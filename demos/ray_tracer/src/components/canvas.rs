use crate::{
    graphics::initialize::build_renderer,
    state::app_context::{AppContext, AppContextError},
};

use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_context, use_effect_with_deps, use_mut_ref, use_node_ref};

#[function_component(Canvas)]
pub fn canvas() -> Html {
    let app_context = use_context::<AppContext>().expect(AppContextError::NOT_FOUND);
    let canvas_ref = use_node_ref();
    let animation_handle = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let app_context = app_context;
            let canvas_ref = canvas_ref.clone();
            let animation_handle = animation_handle;

            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let new_animation_handle = build_renderer(canvas, app_context);
                *animation_handle.borrow_mut() = Some(new_animation_handle);

                move || {}
            }
        },
        (),
    );

    html! { <canvas class="canvas" ref={canvas_ref} /> }
}
