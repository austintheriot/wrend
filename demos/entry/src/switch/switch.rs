use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use hello_quad::components::app::{App as HelloQuadApp};
use hello_quad_animated::components::app::{App as HelloQuadAnimatedApp};
use yew::{html, Html};
use ui::route::Route;

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HelloQuad => html! { <HelloQuadApp /> },
        Route::HelloQuadAnimated => html! { <HelloQuadAnimatedApp /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
