use std::{fmt::Debug, ops::Deref};

use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast, JsValue};
use web_sys::EventTarget;

/// Safe wrapper around eventListener callbacks that cleans them up once the `Listener` struct is dropped
/// For more information, see https://github.com/rustwasm/wasm-bindgen/issues/993.
///
/// This utility can be used with any type that dereferences to EventTarget, so it is not limited
/// to just pure HtmlElements.
#[derive(Debug)]
pub struct Listener<Element: Deref<Target = EventTarget>, Arg: FromWasmAbi + 'static = JsValue> {
    element: Element,
    name: &'static str,
    cb: Closure<dyn Fn(Arg)>,
}

impl<Element: Deref<Target = EventTarget>, Arg: FromWasmAbi + 'static> Listener<Element, Arg> {
    pub fn new<F>(element: Element, name: &'static str, cb: F) -> Self
    where
        F: Fn(Arg) + 'static,
    {
        let cb = Closure::wrap(Box::new(cb) as Box<dyn Fn(Arg)>);

        element
            .add_event_listener_with_callback(name, cb.as_ref().unchecked_ref())
            .unwrap();
        Self { element, name, cb }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn callback(&self) -> &Closure<dyn Fn(Arg)> {
        &self.cb
    }
}

impl<Element: Deref<Target = EventTarget>, Arg: FromWasmAbi + 'static> Drop
    for Listener<Element, Arg>
{
    fn drop(&mut self) {
        self.element
            .remove_event_listener_with_callback(self.name, self.cb.as_ref().unchecked_ref())
            .unwrap();
    }
}
