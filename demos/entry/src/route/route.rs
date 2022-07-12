use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use hello_quad::components::app::{App as HelloQuadApp};
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello_quad")]
    HelloQuad,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HelloQuad => html! { <HelloQuadApp /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
