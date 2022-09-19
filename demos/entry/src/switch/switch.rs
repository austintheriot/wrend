use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use flow_field::components::app::App as FlowFieldApp;
use flow_field_colorful::components::app::App as FlowFieldColorfulApp;
use game_of_life::components::app::App as GameOfLifeApp;
use hello_quad::components::app::App as HelloQuadApp;
use hello_quad_animated::components::app::App as HelloQuadAnimatedApp;
use larger_than_life::components::app::App as LargerThanLifeApp;
use perlin_noise::components::app::App as PerlinNoiseApp;
use ray_tracer::components::app::App as RayTracerApp;
use recording_demo::components::app::App as RecordingDemoApp;
use simplex_noise::components::app::App as SimplexNoiseApp;
use shared::route::Route;
use yew::{html, Html};

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
        Route::HelloQuad => html! { <HelloQuadApp /> },
        Route::HelloQuadAnimated => html! { <HelloQuadAnimatedApp /> },
        Route::GameOfLife => html! { <GameOfLifeApp /> },
        Route::LargerThanLife => html! { <LargerThanLifeApp /> },
        Route::PerlinNoise => html! { <PerlinNoiseApp /> },
        Route::FlowField => html! { <FlowFieldApp /> },
        Route::FlowFieldColorful => html! { <FlowFieldColorfulApp /> },
        Route::SimplexNoise => html! { <SimplexNoiseApp /> },
        Route::RayTracer => html! { <RayTracerApp /> },
        Route::RecordingDemo => html! { <RecordingDemoApp /> },
    }
}
