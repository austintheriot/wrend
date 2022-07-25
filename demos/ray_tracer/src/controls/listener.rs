use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::EventTarget;

/// Safe wrapper around listener callbacks that cleans them up once `Listener` is dropped.
/// For more information, see https://github.com/rustwasm/wasm-bindgen/issues/993
pub struct Listener {
    element: EventTarget,
    name: &'static str,
    cb: Closure<dyn Fn()>,
}

impl Listener {
    pub fn new<F>(element: &EventTarget, name: &'static str, cb: F) -> Self
    where
        F: Fn() + 'static,
    {
        let element = element.to_owned();
        let cb = Closure::wrap(Box::new(cb) as Box<dyn Fn()>);
        element
            .add_event_listener_with_callback(name, cb.as_ref().unchecked_ref())
            .unwrap();
        Self { element, name, cb }
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        self.element
            .remove_event_listener_with_callback(self.name, self.cb.as_ref().unchecked_ref())
            .unwrap();
    }
}
