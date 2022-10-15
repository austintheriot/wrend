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

    use_effect_with_deps(
        {
            let app_context = app_context;
            let canvas_ref = canvas_ref.clone();

            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let new_renderer = build_renderer(canvas, app_context.clone());
                *app_context.renderer.borrow_mut() = Some(new_renderer);

                move || {}
            }
        },
        (),
    );

    html! { <canvas class="canvas" ref={canvas_ref} /> }
}
