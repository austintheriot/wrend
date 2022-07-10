use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use kernels::components::kernels_app::KernelsApp;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/kernels")]
    Kernels,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Kernels => html! { <KernelsApp /> },
        Route::NotFound => html! { <NotFound /> },
    }
}