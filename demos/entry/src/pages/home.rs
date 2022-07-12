use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::route::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
     <>
        <h1>{"Demos"}</h1>
        <Link<Route> to={Route::HelloQuad}>{"Hello Quad"}</Link<Route>>
     </>
    }
}
