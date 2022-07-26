use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{
    controls::{
        make_handle_keydown, make_handle_keyup, make_handle_mouse_move, make_handle_resize,
        make_handle_wheel, Listener,
    },
    state::app_context::{AppContext, AppContextError},
};
use web_sys::{window, EventTarget};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyboardListenerProps {
    pub children: Children,
}

#[function_component(GlobalListeners)]
pub fn global_listeners(props: &KeyboardListenerProps) -> Html {
    let app_context = use_context::<AppContext>().expect(AppContextError::NOT_FOUND);
    let listener_mut_ref: Rc<RefCell<Vec<Box<dyn Any>>>> = use_mut_ref(|| Vec::with_capacity(10));

    use_effect_with_deps(
        move |_| {
            // add global listeners
            let window = window().unwrap();
            let window_event_target: &EventTarget = window.as_ref();

            listener_mut_ref.borrow_mut().push(Box::new(Listener::new(
                window_event_target,
                "wheel",
                make_handle_wheel(app_context.clone()),
            )));

            listener_mut_ref.borrow_mut().push(Box::new(Listener::new(
                window_event_target,
                "keydown",
                make_handle_keydown(app_context.clone()),
            )));

            listener_mut_ref.borrow_mut().push(Box::new(Listener::new(
                window_event_target,
                "keyup",
                make_handle_keyup(app_context.clone()),
            )));

            listener_mut_ref.borrow_mut().push(Box::new(Listener::new(
                window_event_target,
                "resize",
                make_handle_resize(app_context.clone()),
            )));

            listener_mut_ref.borrow_mut().push(Box::new(Listener::new(
                window_event_target,
                "mousemove",
                make_handle_mouse_move(app_context.clone()),
            )));

            || {}
        },
        (),
    );

    html! {
     <>
         {for props.children.iter()}
     </>
    }
}
