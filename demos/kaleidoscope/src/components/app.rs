use crate::graphics::{initialize_renderer, FilterType, InitializeRendererArgs, GenerationType};

use log::error;
use shared::route::Route;
use strum::IntoEnumIterator;

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, Callback};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let generation_select_ref = use_node_ref();
    let filter_select_ref = use_node_ref();
    let render_state_handle_ref = use_mut_ref(|| None);
    let renderer_ref = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let render_state_handle_ref = render_state_handle_ref.clone();
            move |_| {
                let new_renderer = initialize_renderer(InitializeRendererArgs {
                    canvas_ref,
                    render_state_handle_ref,
                });

                // save handle to keep animation going
                *renderer_ref.borrow_mut() = Some(new_renderer);

                || {}
            }
        },
        (),
    );

    let handle_generation_change = {
        let render_state_handle_ref = render_state_handle_ref.clone();
        let generation_select_ref = generation_select_ref.clone();
        Callback::from(move |_: Event| {
            if let Some(render_state_handle) = render_state_handle_ref.borrow().as_ref() {
                let select_element = generation_select_ref
                    .get()
                    .unwrap()
                    .dyn_into::<HtmlSelectElement>()
                    .unwrap();
                let selected_index = select_element.selected_index();

                let generation_type = match GenerationType::iter().nth(selected_index as usize) {
                    Some(generation_type) => generation_type,
                    None => {
                        error!("Unexpected select option reached for generation type: index =  {selected_index}");
                        Default::default()
                    }
                };

                render_state_handle
                    .borrow_mut()
                    .set_generation_type(generation_type);
            }
        })
    };

    let handle_filter_change = {
        let render_state_handle_ref = render_state_handle_ref;
        let filter_select_ref = filter_select_ref.clone();
        Callback::from(move |_: Event| {
            if let Some(render_state_handle) = render_state_handle_ref.borrow().as_ref() {
                let select_element = filter_select_ref
                    .get()
                    .unwrap()
                    .dyn_into::<HtmlSelectElement>()
                    .unwrap();
                let selected_index = select_element.selected_index();

                let filter_type = match FilterType::iter().nth(selected_index as usize) {
                    Some(filter_type) => filter_type,
                    None => {
                        error!("Unexpected select option reached for filter_type: index =  {selected_index}");
                        Default::default()
                    }
                };

                render_state_handle
                    .borrow_mut()
                    .set_filter_type(filter_type);
            }
        })
    };
    

    html! {
        <div class="kaleidoscope">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} width="1000" height="1000" />
            <label for="select-generation">{"Choose a generation"}</label>
            <select
                name="generation"
                id="select-generation"
                onchange={handle_generation_change}
                ref={generation_select_ref}
            >
                {for GenerationType::iter().map(|generation_type| {
                    html!{
                        <option
                            value={generation_type.to_string()}
                            selected={generation_type == GenerationType::default()}
                        >
                            {generation_type.to_string()}
                        </option>
                    }
                })}
            </select>
            <label for="select-filter">{"Choose a filter"}</label>
            <select
                name="filter"
                id="select-filter"
                onchange={handle_filter_change}
                ref={filter_select_ref}
            >
                {for FilterType::iter().map(|filter_type| {
                    html!{
                        <option
                            value={filter_type.to_string()}
                            selected={filter_type == FilterType::default()}
                        >
                            {filter_type.to_string()}
                        </option>
                    }
                })}
            </select>

        </div>
    }
}
