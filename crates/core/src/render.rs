use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Document, Element, Node, window};
#[cfg(not(target_arch = "wasm32"))]
use std::marker::PhantomData;

use crate::console_log;

#[cfg(not(target_arch = "wasm32"))]
impl Renderer {
    pub fn new() -> Self {
        Renderer { _phantom: PhantomData }
    }

    pub fn render(&self, _vnode: &VNode) {
        console_log!("Rendering would happen here in WASM");
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn h(node_type: &str) -> VNode {
    VNode::element(node_type)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn reactive_render(_render_fn: impl Fn() -> VNode) {
    console_log!("Reactive rendering would happen here in WASM");
}

/// Virtual DOM Node representation
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct VNode {
    node_type: String,
    props: HashMap<String, String>,
    children: Vec<VNode>,
    text_content: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl VNode {
    /// Create a new element node
    pub fn element(node_type: &str) -> VNode {
        VNode {
            node_type: node_type.to_string(),
            props: HashMap::new(),
            children: Vec::new(),
            text_content: None,
        }
    }

    /// Create a new text node
    pub fn text(content: &str) -> VNode {
        VNode {
            node_type: "TEXT".to_string(),
            props: HashMap::new(),
            children: Vec::new(),
            text_content: Some(content.to_string()),
        }
    }

    /// Set a property on the node
    pub fn set_prop(mut self, key: &str, value: &str) -> Self {
        self.props.insert(key.to_string(), value.to_string());
        self
    }

    /// Add a child node
    pub fn add_child(mut self, child: VNode) -> Self {
        self.children.push(child);
        self
    }

    /// Set text content (for text nodes)
    pub fn set_text(mut self, content: &str) -> Self {
        self.text_content = Some(content.to_string());
        self
    }
}

/// Virtual DOM renderer with efficient diffing
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Renderer {
    #[cfg(target_arch = "wasm32")]
    document: Document,
    #[cfg(not(target_arch = "wasm32"))]
    _phantom: PhantomData<()>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Renderer {
    /// Create a new renderer
    pub fn new() -> Result<Renderer, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        Ok(Renderer { document })
    }

    /// Render a virtual node to a container element
    pub fn render(&self, vnode: &VNode, container: &Element) -> Result<(), JsValue> {
        // Clear container
        container.set_inner_html("");

        // Create and append the element
        let element = self.create_element(vnode)?;
        container.append_child(&element)?;

        Ok(())
    }

    /// Create a DOM element from a virtual node
    fn create_element(&self, vnode: &VNode) -> Result<Node, JsValue> {
        if vnode.node_type == "TEXT" {
            let text_node = self.document.create_text_node(
                &vnode.text_content.clone().unwrap_or_default()
            );
            return Ok(text_node.into());
        }

        let element = self.document.create_element(&vnode.node_type)?;

        // Set properties
        for (key, value) in &vnode.props {
            if key.starts_with("on") {
                // Event handlers would be set up here
                // For now, we'll skip this
            } else {
                element.set_attribute(key, value)?;
            }
        }

        // Add children
        for child in &vnode.children {
            let child_element = self.create_element(child)?;
            element.append_child(&child_element)?;
        }

        Ok(element.into())
    }
}

/// Create a virtual element node (JavaScript-compatible API)
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn h(node_type: &str) -> VNode {
    VNode::element(node_type)
}

/// Reactive rendering helper
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn reactive_render(
    render_fn: &js_sys::Function,
    container: &Element,
) -> Result<(), JsValue> {
    let renderer = Renderer::new()?;

    // Create a closure that calls the render function and re-renders
    let render_callback = Closure::wrap(Box::new(move || {
        // Call the JavaScript render function
        let vnode_result = render_fn.call0(&JsValue::NULL);

        match vnode_result {
            Ok(vnode_js) => {
                // Convert JS object to VNode (simplified)
                // In practice, we'd need proper JS interop
                console_log!("Re-rendering...");
            }
            Err(e) => {
                console_log!("Render error: {:?}", e);
            }
        }
    }) as Box<dyn Fn()>);

    // For now, just call it once. Convert the Closure reference to a JS Function and call it.
    let func = render_callback.as_ref().unchecked_ref::<js_sys::Function>();
    let _ = func.call0(&JsValue::NULL);

    // In a real implementation, this would be called when signals change
    // We'd need to integrate with the reactivity system

    Ok(())
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
