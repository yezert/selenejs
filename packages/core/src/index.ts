import { h, reactiveRender } from './render'
export { signal, computed, effect } from './reactivity'
export { render, reactiveRender, h } from './render'

// Import Rust/WASM bindings
// The WASM bundle is generated into `src/rust/selene_core.js` by the build step.
// TypeScript may not see the generated file during dev checks, so ignore the import error.
// @ts-ignore
import init, { JsSignal as RustSignal, compile_template as rust_compile_template } from './rust/selene_core.js'

// Initialize WASM module
let wasmInitialized = false
let wasmInitAttempted = false
async function initWasm() {
  if (!wasmInitialized) {
    wasmInitAttempted = true
    // wasm-pack's init attempts to `fetch()` the wasm file when given a URL.
    // In Node, fetch on file:// is not supported by undici, so read the
    // .wasm bytes directly and pass them to the initializer when running
    // under Node.js. In browsers, pass the URL so `fetch` works as expected.
    const wasmUrl = new URL('./rust/selene_core_bg.wasm', import.meta.url);
    const isNode = typeof globalThis !== 'undefined' && (globalThis as any).process && (globalThis as any).process.versions && (globalThis as any).process.versions.node;
    if (isNode) {
      // @ts-ignore - dynamic import of Node `fs` at runtime
      const fs: any = await import('fs');
      const bytes = fs.readFileSync(wasmUrl);
      await init({ module_or_path: bytes });
    } else {
      await init(wasmUrl);
    }
    wasmInitialized = true
  }
}

// JSX support
export function jsx(type: any, props: any) {
  const { children, ...restProps } = props || {}
  return h(type, restProps, children)
}

export function jsxs(type: any, props: any) {
  return jsx(type, props)
}

export const Fragment = Symbol.for('selene.fragment')

// Export enhanced functionality when WASM is available
export async function createRustSignal(initialValue: number) {
  await initWasm()
  return new RustSignal(initialValue)
}

// Expose a simple Rust-based compiler (WASM) to JS
export async function compileTemplate(input: string) {
  // Prefer Rust/WASM compiler, but never let app hang on wasm init.
  // If wasm init fails or times out, fall back to JS compiler (best-effort).
  try {
    const initPromise = initWasm()
    await Promise.race([
      initPromise,
      new Promise((_, rej) =>
        setTimeout(() => rej(new Error('WASM init timeout')), 500)
      ),
    ])
    // @ts-ignore
    return rust_compile_template(input)
  } catch {
    return compileTemplateJS(input)
  }
}

function compileTemplateJS(input: string) {
  // Browser DOMParser-based implementation
  try {
    const parser = new DOMParser()
    const doc = parser.parseFromString(input, 'text/html')
    const root = doc.body.firstElementChild
    if (!root) return "() => (\"\")"

    function serialize(node: Element | ChildNode): string {
      // Element
      // @ts-ignore
      if (node.nodeType === 1) {
        const el = node as Element
        const tag = el.tagName.toLowerCase()
        const attrs: string[] = []
        for (const a of Array.from(el.attributes)) {
          const name = a.name
          const val = a.value
          if (val.startsWith('{') && val.endsWith('}')) {
            attrs.push(`\"${name}\": ${val.slice(1, -1)}`)
          } else {
            attrs.push(`\"${name}\": \"${val.replace(/\"/g, '\\\\"')}\"`)
          }
        }
        const attrsJs = attrs.length ? `{${attrs.join(', ')}}` : '{}'
        const children: string[] = []
        for (const c of Array.from(el.childNodes)) {
          const s = serialize(c)
          if (s) children.push(s)
        }
        if (children.length === 0) {
          return `h(\"${tag}\", ${attrsJs})`
        } else if (children.length === 1) {
          return `h(\"${tag}\", ${attrsJs}, ${children[0]})`
        } else {
          return `h(\"${tag}\", ${attrsJs}, [${children.join(', ')}])`
        }
      }

      // Text node
      const txt = node.textContent || ''
      const t = txt.trim()
      if (!t) return ''
      if (t.includes('${')) {
        const lit = t.replace(/`/g, '\\`')
        return `\`${lit}\``
      }
      return `\"${t.replace(/\"/g, '\\\\"')}\"`
    }

    const expr = serialize(root)
    return `() => ${expr}`
  } catch (e) {
    // Last-resort naive fallback: return text
    const escaped = input.replace(/"/g, '\\"')
    return `() => ("${escaped}")`
  }
}

export type { RustSignal }

// 小而新的 API：类 Vue 的 createApp，封装 reactiveRender，简化用户使用。
export function createApp(view: () => any) {
  return {
    mount(container: HTMLElement | string | null | undefined) {
      const el =
        typeof container === 'string'
          ? (document.querySelector(container) as HTMLElement | null)
          : (container as HTMLElement | null)
      if (!el) {
        throw new Error('[selene] mount target not found')
      }
      reactiveRender(view, el)
    },
  }
}
