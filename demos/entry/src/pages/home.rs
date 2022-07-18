use ui::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
     <>
        <h1>{"Demos"}</h1>
        <Link<Route> to={Route::HelloQuad}>{"Hello Quad"}</Link<Route>>
        <Link<Route> to={Route::HelloQuadAnimated}>{"Hello Quad Animated"}</Link<Route>>
        <Link<Route> to={Route::GameOfLife}>{"Game of Life"}</Link<Route>>
        <Link<Route> to={Route::LargerThanLife}>{"Larger Than Life"}</Link<Route>>
        <Link<Route> to={Route::PerlinNoise}>{"Perlin Noise"}</Link<Route>>
        <Link<Route> to={Route::FlowField}>{"Flow Field"}</Link<Route>>
     </>
    }
}
