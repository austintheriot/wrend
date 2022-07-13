use yew::prelude::*;
use yew_router::prelude::*;
use ui::route::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
     <>
        <h1>{"Demos"}</h1>
        <Link<Route> to={Route::HelloQuad}>{"Hello Quad"}</Link<Route>>
        <Link<Route> to={Route::HelloQuadAnimated}>{"Hello Quad Animated"}</Link<Route>>
        <Link<Route> to={Route::GameOfLife}>{"Game of Life"}</Link<Route>>
     </>
    }
}
