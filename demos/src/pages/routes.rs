use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::test::Test;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Test => html! { <Test /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
