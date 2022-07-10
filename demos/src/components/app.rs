use yew::{function_component, html};
use yew_router::prelude::*;
use crate::routes::routes::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
