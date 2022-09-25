use std::rc::Rc;

use crate::{
    graphics::{initialize_renderer, FilterType, GenerationType, InitializeRendererArgs},
    state::UiState,
};

use log::error;
use shared::route::Route;
use strum::IntoEnumIterator;

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement, MouseEvent};

use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, use_state_eq,
    Callback, UseStateHandle,
};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // element refs
    let canvas_ref = use_node_ref();
    let generation_select_ref = use_node_ref();

    // state handles (for setting)
    let generation_type = use_state_eq(GenerationType::default);
    let applied_filters = use_state_eq(Vec::new);

    // state refs (for getting)
    let generation_type_ref = use_mut_ref(GenerationType::default);
    let applied_filters_ref = use_mut_ref(Vec::default);

    let app_state_handle_ref = use_mut_ref(|| None);
    let renderer_ref = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let generation_type_ref = Rc::clone(&generation_type_ref);
            let applied_filters_ref = Rc::clone(&applied_filters_ref);
            move |(applied_filters, generation_type): &(
                UseStateHandle<Vec<FilterType>>,
                UseStateHandle<GenerationType>,
            )| {
                *generation_type_ref.borrow_mut() = **generation_type;
                *applied_filters_ref.borrow_mut() = (**applied_filters).clone();

                || {}
            }
        },
        (applied_filters.clone(), generation_type.clone()),
    );

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let app_state_handle_ref = app_state_handle_ref;
            let applied_filters_ref = applied_filters_ref;
            let generation_type = generation_type.clone();
            let applied_filters = applied_filters.clone();
            move |_| {
                let ui_state = UiState::new(
                    generation_type_ref,
                    applied_filters_ref,
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

    let make_handle_add_filter_click = {
        let applied_filters = applied_filters.clone();
        move |filter_type: FilterType| {
            let applied_filters = applied_filters.clone();
            Callback::from(move |_: MouseEvent| {
                let mut new_applied_filters = (*applied_filters).clone();
                new_applied_filters.push(filter_type);
                applied_filters.set(new_applied_filters);
            })
        }
    };

    let make_handle_remove_filter_click = {
        let applied_filters = applied_filters.clone();
        move |i: usize| {
            let applied_filters = applied_filters.clone();
            Callback::from(move |_: MouseEvent| {
                let mut new_applied_filters = (*applied_filters).clone();
                new_applied_filters.remove(i);
                applied_filters.set(new_applied_filters);
            })
        }
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


            <p>{"Add a Filter: "}</p>
           {for FilterType::iter().map(|filter_type_el| {
            html!{
                <button onclick={make_handle_add_filter_click(filter_type_el)}>
                    {filter_type_el.to_string()}
                </button>
            }
            })}

            <p>{"Currently Set Filters: "}</p>
            {if applied_filters.is_empty() {
                html!{ <p>{"No filters selected"}</p>}
            } else {
               html!{
                <>
                    {for applied_filters.iter().enumerate().map(|(i, filter_type_el)| {
                        html!{
                            <button onclick={make_handle_remove_filter_click(i)}>
                                {filter_type_el.to_string()}
                            </button>
                        }
                    })}
                </>
               }
            }}
        </div>
    }
}