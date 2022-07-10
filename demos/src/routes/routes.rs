use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::test::Test;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/webgl-rust/")]
    Home,
    #[at("/webgl-rust/test")]
    Test,
    #[not_found]
    #[at("/webgl-rust/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Test => html! { <Test /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
