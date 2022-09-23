use std::rc::Rc;

use crate::{
    graphics::{initialize_renderer, FilterType, GenerationType, InitializeRendererArgs},
    state::UiState,
};

use log::error;
use shared::route::Route;
use strum::IntoEnumIterator;

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, use_state_eq,
    Callback, UseStateHandle,
};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let generation_select_ref = use_node_ref();
    let filter_select_ref = use_node_ref();
    let app_state_handle_ref = use_mut_ref(|| None);
    let renderer_ref = use_mut_ref(|| None);
    let generation_type = use_state_eq(GenerationType::default);
    let filter_type = use_state_eq(FilterType::default);
    let applied_filters = use_state_eq(Vec::new);
    let generation_type_ref = use_mut_ref(GenerationType::default);
    let filter_type_ref = use_mut_ref(FilterType::default);

    use_effect_with_deps(
        {
            let generation_type_ref = Rc::clone(&generation_type_ref);
            let filter_type_ref = Rc::clone(&filter_type_ref);
            move |(filter_type, generation_type): &(
                UseStateHandle<FilterType>,
                UseStateHandle<GenerationType>,
            )| {
                *generation_type_ref.borrow_mut() = **generation_type;
                *filter_type_ref.borrow_mut() = **filter_type;

                || {}
            }
        },
        (filter_type.clone(), generation_type.clone()),
    );

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let app_state_handle_ref = app_state_handle_ref.clone();
            let generation_type = generation_type.clone();
            let filter_type = filter_type.clone();
            let applied_filters = applied_filters.clone();
            move |_| {
                let ui_state = UiState::new(
                    filter_type_ref,
                    generation_type_ref,
                    filter_type,
                    generation_type,
                    applied_filters,
                );

                let new_renderer = initialize_renderer(InitializeRendererArgs {
                    canvas_ref,
                    app_state_handle_ref,
                    ui_state,
                });

                // save handle to keep animation going
                *renderer_ref.borrow_mut() = Some(new_renderer);

                || {}
            }
        },
        (),
    );

    let handle_generation_change = {
        let generation_type = generation_type.clone();
        let generation_select_ref = generation_select_ref.clone();
        Callback::from(move |_: Event| {
            let select_element = generation_select_ref
                .get()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            let selected_index = select_element.selected_index();

            let new_generation_type = match GenerationType::iter().nth(selected_index as usize) {
                Some(new_generation_type) => new_generation_type,
                None => {
                    error!("Unexpected select option reached for generation type: index =  {selected_index}");
                    Default::default()
                }
            };

            generation_type.set(new_generation_type);
        })
    };

    let handle_filter_change = {
        let filter_type = filter_type.clone();
        let filter_select_ref = filter_select_ref.clone();
        Callback::from(move |_: Event| {
            let select_element = filter_select_ref
                .get()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            let selected_index = select_element.selected_index();

            let new_filter_type = match FilterType::iter().nth(selected_index as usize) {
                Some(new_filter_type) => new_filter_type,
                None => {
                    error!("Unexpected select option reached for filter_type: index =  {selected_index}");
                    Default::default()
                }
            };

            filter_type.set(new_filter_type);
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
                {for GenerationType::iter().map(|generation_type_el| {
                    html!{
                        <option
                            value={generation_type_el.to_string()}
                            selected={generation_type_el == *generation_type}
                        >
                            {generation_type_el.to_string()}
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
                {for FilterType::iter().map(|filter_type_el| {
                    html!{
                        <option
                            value={filter_type_el.to_string()}
                            selected={filter_type_el == *filter_type}
                        >
                            {filter_type_el.to_string()}
                        </option>
                    }
                })}
            </select>

        </div>
    }
}
