#![warn(clippy::all, rust_2018_idioms)]

mod app;
use std::{cell::RefCell, rc::Rc};

pub use app::TemplateApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<CommBridge, JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();
    // Here we'll create a communication bridge that both js and rust possess
    let bridge = CommBridge::new();
    let bridge_clone = bridge.clone();

    eframe::start_web(
        canvas_id,
        // Give a clone of the comm bridge to our app
        Box::new(|cc| Box::new(TemplateApp::new(cc, bridge_clone))),
    )?;
    // Return the comm bridge to js
    Ok(bridge)
}

/// We have to make sure that this type is a cloneable handle to a shared object
#[wasm_bindgen]
#[derive(Clone)]
pub struct CommBridge {
    inner: Rc<RefCell<CommBridgeInner>>,
}

/// Simple comm bridge for a chat-like application
struct CommBridgeInner {
    to_js: Vec<String>,
    from_js: Vec<String>,
}

impl CommBridge {
    fn push_to_js(&self, msg: &str) {
        self.inner.borrow_mut().to_js.push(msg.to_owned());
    }
    fn pull_from_js(&self) -> Option<String> {
        self.inner.borrow_mut().from_js.pop()
    }
    fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(CommBridgeInner {
                to_js: Vec::new(),
                from_js: Vec::new(),
            })),
        }
    }
}

/// Expose methods to js
#[wasm_bindgen]
impl CommBridge {
    pub fn pull(&self) -> JsValue {
        match self.inner.borrow_mut().to_js.pop() {
            Some(val) => JsValue::from_str(&val),
            None => JsValue::null(),
        }
    }
    pub fn push(&self, content: JsValue) {
        if let Some(s) = content.as_string() {
            self.inner.borrow_mut().from_js.push(s);
        }
    }
}
