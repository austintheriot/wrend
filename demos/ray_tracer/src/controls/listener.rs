use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast, JsValue};
use web_sys::EventTarget;

/// Safe wrapper around listener callbacks that cleans them up once `Listener` is dropped.
/// For more information, see https://github.com/rustwasm/wasm-bindgen/issues/993
pub struct Listener<Arg: FromWasmAbi + 'static = JsValue> {
    element: EventTarget,
    name: &'static str,
    cb: Closure<dyn Fn(Arg)>,
}

impl<Arg: FromWasmAbi + 'static> Listener<Arg> {
    pub fn new<F>(element: &EventTarget, name: &'static str, cb: F) -> Self
    where
        F: Fn(Arg) + 'static,
    {
        let element = element.to_owned();
        let cb = Closure::wrap(Box::new(cb) as Box<dyn Fn(Arg)>);
        element
            .add_event_listener_with_callback(name, cb.as_ref().unchecked_ref())
            .unwrap();
        Self { element, name, cb }
    }
}

impl<Arg: FromWasmAbi + 'static> Drop for Listener<Arg> {
    fn drop(&mut self) {
        self.element
            .remove_event_listener_with_callback(self.name, self.cb.as_ref().unchecked_ref())
            .unwrap();
    }
}
