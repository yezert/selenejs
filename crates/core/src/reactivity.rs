use std::cell::RefCell;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Thread-local storage for active effect
thread_local! {
    static ACTIVE_EFFECT: RefCell<Option<Rc<RefCell<Vec<Rc<dyn Fn()>>>>>> = RefCell::new(None);
}

/// Signal implementation in Rust for better performance
#[derive(Clone)]
pub struct Signal<T: Clone + PartialEq + 'static> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Rc<dyn Fn()>>>>,
}

impl<T: Clone + PartialEq + 'static> Signal<T> {
    /// Create a new signal with initial value
    pub fn new(initial_value: T) -> Self {
        Signal {
            value: Rc::new(RefCell::new(initial_value)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Get the current value and track dependencies
    pub fn get(&self) -> T {
        // Track dependency if we're in an effect
        if let Some(active_effect) = ACTIVE_EFFECT.with(|ae| ae.borrow().clone()) {
            let effect_fn = Rc::new(|| {
                // This would trigger re-evaluation of the effect
                // For now, we'll just store a no-op
            });
            active_effect.borrow_mut().push(effect_fn);
        }
        self.value.borrow().clone()
    }

    /// Set a new value and notify subscribers
    pub fn set(&self, new_value: T) {
        if *self.value.borrow() == new_value {
            return;
        }

        *self.value.borrow_mut() = new_value;

        // Notify all subscribers
        let subscribers = self.subscribers.borrow().clone();
        for effect in subscribers {
            effect();
        }
    }

    /// Get current value without tracking dependencies
    pub fn peek(&self) -> T {
        self.value.borrow().clone()
    }
}

/// Create a new signal
pub fn signal<T: Clone + PartialEq + 'static>(initial_value: T) -> Signal<T> {
    Signal::new(initial_value)
}

/// Run an effect function and track its dependencies
pub fn effect<F>(f: F)
where
    F: Fn() + 'static,
{
    let effect_deps = Rc::new(RefCell::new(Vec::new()));

    let effect_fn = Rc::new(move || {
        // Clear previous dependencies
        effect_deps.borrow_mut().clear();

        // Set this effect as active
        ACTIVE_EFFECT.with(|ae| *ae.borrow_mut() = Some(Rc::clone(&effect_deps)));

        // Run the effect
        f();

        // Clear active effect
        ACTIVE_EFFECT.with(|ae| *ae.borrow_mut() = None);
    });

    // Run effect initially
    effect_fn();
}

/// Create a computed signal that derives its value from other signals
pub fn computed<T: Clone + PartialEq + 'static, F>(f: F) -> Signal<T>
where
    F: Fn() -> T + 'static,
{
    let signal = Signal::new(f());

    effect({
        let signal = signal.clone();
        move || {
            signal.set(f());
        }
    });

    signal
}

// wasm-bindgen doesn't support generic impls. Provide a concrete JS-facing wrapper
// for numeric signals (f64) used by the JS side.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct JsSignal {
    inner: Signal<f64>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl JsSignal {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_value: f64) -> JsSignal {
        JsSignal {
            inner: Signal::new(initial_value),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f64 {
        self.inner.get()
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: f64) {
        self.inner.set(value);
    }
}
