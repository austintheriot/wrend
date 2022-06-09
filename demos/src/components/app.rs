use yew::{function_component, html, prelude::*};
use crate::state::app_context::AppContext;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ContextProvider<AppContext> context={AppContext::default()}>
            <p>{"Test 1"}</p>
        </ContextProvider<AppContext>>
    }
}
