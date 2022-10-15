use std::rc::Rc;

use crate::{
    graphics::{initialize_renderer, FilterType, GenerationType, InitializeRendererArgs},
    state::UiState,
};

use log::error;
use shared::{route::Route, SharedClass};
use strum::IntoEnumIterator;

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement, MouseEvent};

use yew::{
    classes, function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref,
    use_state_eq, Callback, UseStateHandle,
};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // element refs
    let canvas_ref = use_node_ref();
    let video_ref = use_node_ref();
    let generation_select_ref = use_node_ref();

    // state handles (for setting)
    let generation_type = use_state_eq(GenerationType::default);
    let applied_filters = use_state_eq(Vec::new);
    let is_recording = use_state_eq(bool::default);

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
            let video_ref = video_ref.clone();
            let app_state_handle_ref = Rc::clone(&app_state_handle_ref);
            let applied_filters_ref = Rc::clone(&applied_filters_ref);
            let renderer_ref = Rc::clone(&renderer_ref);
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
                    video_ref,
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

    let handle_save_image = {
        let app_state_handle_ref = Rc::clone(&app_state_handle_ref);
        Callback::from(move |_: MouseEvent| {
            if let Some(app_state_handle_ref) = &*app_state_handle_ref.borrow() {
                app_state_handle_ref.borrow_mut().set_should_save(true);
            }
        })
    };

    let handle_clear_all_filters = {
        let applied_filters = applied_filters.clone();
        Callback::from(move |_: MouseEvent| {
            applied_filters.set(Vec::new());
        })
    };

    let handle_start_recording = {
        let renderer_ref = Rc::clone(&renderer_ref);
        let is_recording = is_recording.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(renderer) = &mut *renderer_ref.borrow_mut() {
                renderer.start_recording();
                is_recording.set(true);
            }
        })
    };

    let handle_stop_recording = {
        let renderer_ref = Rc::clone(&renderer_ref);
        let is_recording = is_recording.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(renderer) = &mut *renderer_ref.borrow_mut() {
                renderer.stop_recording();
                is_recording.set(false);
            }
        })
    };

    let handle_clear_recorded_data = {
        let renderer_ref = Rc::clone(&renderer_ref);
        let _is_recording = is_recording.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(renderer) = &mut *renderer_ref.borrow_mut() {
                renderer.clear_recorded_data();
            }
        })
    };

    // hide video element when not using video as input
    let video_style = if *generation_type == GenerationType::VideoInput {
        ""
    } else {
        "display: none;"
    };

    html! {
        <div class="kaleidoscope">
            <div class="quick-access-container">
                <div class="quick-access-buttons">
                    <Link<Route> to={Route::Home} classes={classes!(SharedClass::Button.to_string())}>{"Home"}</Link<Route>>
                    <button onclick={handle_save_image} class={SharedClass::Button.to_string()}>
                        {"Save Image"}
                    </button>
                    {if !*is_recording {
                        html!{
                            <button onclick={handle_start_recording} class={SharedClass::Button.to_string()}>
                                {"Start Recording"}
                            </button>
                        }
                    } else {
                        html!{
                            <button onclick={handle_stop_recording} class={SharedClass::Button.to_string()}>
                                {"Stop Recording"}
                            </button>
                        }
                    }}

                    <button onclick={handle_clear_recorded_data} class={SharedClass::Button.to_string()}>
                        {"Clear Recorded Data"}
                    </button>
                </div>

                <details>
                    <summary>{"Choose input"}</summary>
                    <label for="select-generation" class="generation-select-label">{"Choose a generation"}</label>
                    <select
                        name="generation"
                        id="select-generation"
                        class="generation-select"
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
                </details>

                <details>
                    <summary>{"Add a filter"}</summary>
                    <div class="add-filter-button-container">
                        {for FilterType::iter().map(|filter_type_el| {
                            html!{
                                <button onclick={make_handle_add_filter_click(filter_type_el)} class={SharedClass::Button.to_string()}>
                                    {filter_type_el.to_string()}
                                </button>
                            }
                        })}
                    </div>
                </details>

                <details>
                    <summary>{"Remove filters"}</summary>
                    <button
                        onclick={handle_clear_all_filters}
                        class={classes!(SharedClass::Button.to_string(), "clear-all-filters-button")}>
                        {"Clear All Filters"}
                    </button>

                    <p>{"Currently Set Filters: "}</p>
                    <div class="remove-filter-button-container">
                        {if applied_filters.is_empty() {
                            html!{ <p>{"No filters selected"}</p>}
                        } else {
                            html!{
                                {for applied_filters.iter().enumerate().map(|(i, filter_type_el)| {
                                    html!{
                                        <button onclick={make_handle_remove_filter_click(i)} class={SharedClass::Button.to_string()}>
                                            {filter_type_el.to_string()}
                                        </button>
                                    }
                                })}
                            }
                        }}
                    </div>
                </details>
            </div>

            <canvas ref={canvas_ref} width="2500" height="2500" />
            <video controls=true ref={video_ref} src="./big_buck_bunny.mp4" style={video_style} />
        </div>
    }
}
