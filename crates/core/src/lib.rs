mod reactivity;
mod render;
mod compiler;

pub use reactivity::*;
pub use render::*;
pub use compiler::compile_template;

// WebAssembly bindings
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Re-export for JavaScript
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Single console_log macro definition with conditional compilation
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        {
            crate::log(&format_args!($($t)*).to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            println!($($t)*)
        }
    }
}
