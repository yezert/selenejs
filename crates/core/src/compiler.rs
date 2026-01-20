#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Template compiler:
/// - Input: HTML-ish template string (single or multiple root nodes)
/// - Output: JS function source string, e.g. `() => h("div", {...}, [...])`
///
/// Design goals:
/// - Deterministic output (no DOM dependency)
/// - Works in wasm32 + non-wasm builds
/// - Handles nested tags + self-closing tags + attributes
/// - Text nodes preserve content (whitespace-only nodes dropped)
pub fn compile_template_sync(input: &str) -> String {
    let mut p = Parser::new(input);
    p.skip_ws();
    let nodes = p.parse_nodes();
    let expr = emit_nodes(&nodes);
    format!("() => {}", expr)
}

#[derive(Debug, Clone)]
enum Node {
    Element(ElementNode),
    Text(String),
}

#[derive(Debug, Clone)]
struct ElementNode {
    tag: String,
    attrs: Vec<(String, AttrValue)>,
    children: Vec<Node>,
    self_closing: bool,
}

#[derive(Debug, Clone)]
enum AttrValue {
    BoolTrue,
    Str(String),
    JsExpr(String),
}

fn emit_nodes(nodes: &[Node]) -> String {
    if nodes.is_empty() {
        return "(\"\")".to_string();
    }
    if nodes.len() == 1 {
        return emit_node(&nodes[0]);
    }
    // Multiple roots: wrap in Fragment so runtime can accept a single VNode.
    // NOTE: This assumes `Fragment` is in scope where the generated function runs.
    let children = nodes.iter().map(emit_node).collect::<Vec<_>>().join(", ");
    format!("h(Fragment, {{}}, [{}])", children)
}

fn emit_node(n: &Node) -> String {
    match n {
        Node::Text(t) => emit_text(t),
        Node::Element(el) => emit_element(el),
    }
}

fn emit_text(t: &str) -> String {
    // If text contains ${...}, keep it as a template literal.
    if t.contains("${") {
        let lit = t.replace('`', "\\`");
        return format!("`{}`", lit);
    }
    js_string(t)
}

fn emit_element(el: &ElementNode) -> String {
    let tag_js = js_string(&el.tag);
    let attrs_js = emit_attrs(&el.attrs);
    if el.self_closing || el.children.is_empty() {
        return format!("h({}, {})", tag_js, attrs_js);
    }
    let children = el.children.iter().map(emit_node).collect::<Vec<_>>();
    if children.len() == 1 {
        format!("h({}, {}, {})", tag_js, attrs_js, children[0])
    } else {
        format!("h({}, {}, [{}])", tag_js, attrs_js, children.join(", "))
    }
}

fn emit_attrs(attrs: &[(String, AttrValue)]) -> String {
    if attrs.is_empty() {
        return "{}".to_string();
    }
    let mut parts = Vec::new();
    for (k, v) in attrs {
        let key = js_string(k);
        let val = match v {
            AttrValue::BoolTrue => "true".to_string(),
            AttrValue::Str(s) => js_string(s),
            AttrValue::JsExpr(expr) => expr.trim().to_string(),
        };
        parts.push(format!("{}: {}", key, val));
    }
    format!("{{{}}}", parts.join(", "))
}

fn js_string(s: &str) -> String {
    // Emit a JS string literal with minimal escaping.
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

struct Parser<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, i: 0 }
    }

    fn eof(&self) -> bool {
        self.i >= self.s.len()
    }

    fn rest(&self) -> &'a str {
        &self.s[self.i..]
    }

    fn peek_char(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.i += ch.len_utf8();
        Some(ch)
    }

    fn starts_with(&self, pat: &str) -> bool {
        self.rest().starts_with(pat)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_char(), Some(c) if c.is_whitespace()) {
            self.bump_char();
        }
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while !self.eof() {
            if self.starts_with("</") {
                break;
            }
            if self.starts_with("<") {
                if let Some(el) = self.parse_element() {
                    nodes.push(Node::Element(el));
                    continue;
                }
                // If we can't parse an element, treat '<' as text.
                nodes.push(Node::Text(self.collect_text_until_tag()));
                continue;
            }
            let t = self.collect_text_until_tag();
            if !t.trim().is_empty() {
                nodes.push(Node::Text(t));
            }
        }
        nodes
    }

    fn collect_text_until_tag(&mut self) -> String {
        let mut out = String::new();
        while !self.eof() {
            if self.starts_with("<") {
                break;
            }
            if let Some(ch) = self.bump_char() {
                out.push(ch);
            }
        }
        out
    }

    fn parse_element(&mut self) -> Option<ElementNode> {
        if !self.starts_with("<") || self.starts_with("</") {
            return None;
        }
        self.bump_char()?; // '<'
        self.skip_ws();
        let tag = self.parse_tag_name();
        if tag.is_empty() {
            return None;
        }
        let attrs = self.parse_attributes();
        self.skip_ws();

        // self-closing?
        if self.starts_with("/>") {
            self.i += 2;
            return Some(ElementNode {
                tag,
                attrs,
                children: vec![],
                self_closing: true,
            });
        }

        // end of open tag
        if self.starts_with(">") {
            self.i += 1;
        } else {
            // malformed, bail out
            return None;
        }

        let is_void = is_void_element(&tag);

        // parse children
        let mut children = Vec::new();
        if !is_void {
            children = self.parse_nodes();
            // consume close tag if present
            if self.starts_with("</") {
                self.i += 2; // '</'
                self.skip_ws();
                let close_tag = self.parse_tag_name();
                // skip until '>'
                while !self.eof() && !self.starts_with(">") {
                    self.bump_char();
                }
                if self.starts_with(">") {
                    self.i += 1;
                }
                // If close tag doesn't match, we still continue (best-effort).
                let _ = close_tag;
            }
        }

        Some(ElementNode {
            tag,
            attrs,
            children,
            self_closing: is_void,
        })
    }

    fn parse_tag_name(&mut self) -> String {
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == ':' {
                out.push(ch);
                self.bump_char();
            } else {
                break;
            }
        }
        out
    }

    fn parse_attributes(&mut self) -> Vec<(String, AttrValue)> {
        let mut attrs = Vec::new();
        loop {
            self.skip_ws();
            if self.eof() || self.starts_with(">") || self.starts_with("/>") {
                break;
            }
            let raw_name = self.parse_attr_name();
            if raw_name.is_empty() {
                // Skip one char to avoid infinite loop on junk
                self.bump_char();
                continue;
            }
            self.skip_ws();
            if self.starts_with("=") {
                self.i += 1;
                self.skip_ws();
                let val = self.parse_attr_value();
                let norm = normalize_attr_name(&raw_name);
                attrs.push((norm, val));
            } else {
                let norm = normalize_attr_name(&raw_name);
                attrs.push((norm, AttrValue::BoolTrue));
            }
        }
        attrs
    }

    fn parse_attr_name(&mut self) -> String {
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() || ch == '=' || ch == '>' || ch == '/' {
                break;
            }
            out.push(ch);
            self.bump_char();
        }
        out
    }

    fn parse_attr_value(&mut self) -> AttrValue {
        if self.starts_with("{") {
            self.i += 1;
            let expr = self.collect_until_matching_brace();
            return AttrValue::JsExpr(expr);
        }
        if self.starts_with("\"") {
            self.i += 1;
            let s = self.collect_until_quote('"');
            if self.starts_with("\"") {
                self.i += 1;
            }
            return AttrValue::Str(s);
        }
        if self.starts_with("'") {
            self.i += 1;
            let s = self.collect_until_quote('\'');
            if self.starts_with("'") {
                self.i += 1;
            }
            return AttrValue::Str(s);
        }
        // unquoted token
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() || ch == '>' {
                break;
            }
            if self.starts_with("/>") {
                break;
            }
            out.push(ch);
            self.bump_char();
        }
        AttrValue::Str(out)
    }

    fn collect_until_quote(&mut self, q: char) -> String {
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == q {
                break;
            }
            out.push(ch);
            self.bump_char();
        }
        out
    }

    fn collect_until_matching_brace(&mut self) -> String {
        // Collect until the first '}' (best-effort; no nested brace support for now).
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == '}' {
                self.i += 1;
                break;
            }
            out.push(ch);
            self.bump_char();
        }
        out
    }
}

fn normalize_attr_name(name: &str) -> String {
    // React 风格：onClick
    // Vue 风格：on:click / @click
    if let Some(rest) = name.strip_prefix("on:") {
        return format!("on{}", capitalize(rest));
    }
    if let Some(rest) = name.strip_prefix('@') {
        return format!("on{}", capitalize(rest));
    }
    name.to_string()
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn compile_template(input: &str) -> String {
    compile_template_sync(input)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn compile_template(input: &str) -> String {
    compile_template_sync(input)
}
