use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use flow_field::components::app::App as FlowFieldApp;
use game_of_life::components::app::App as GameOfLifeApp;
use hello_quad::components::app::App as HelloQuadApp;
use hello_quad_animated::components::app::App as HelloQuadAnimatedApp;
use larger_than_life::components::app::App as LargerThanLifeApp;
use perlin_noise::components::app::App as PerlinNoiseApp;
use ui::route::Route;
use yew::{html, Html};

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
        Route::HelloQuad => html! { <HelloQuadApp /> },
        Route::HelloQuadAnimated => html! { <HelloQuadAnimatedApp /> },
        Route::GameOfLife => html! { <GameOfLifeApp /> },
        Route::LargerThanLife => html! { <LargerThanLifeApp /> },
        Route::FlowField => html! { <FlowFieldApp /> },
        Route::PerlinNoise => html! { <PerlinNoiseApp /> },
    }
}
