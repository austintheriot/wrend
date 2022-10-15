use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use flow_field::components::app::App as FlowFieldApp;
use game_of_life::components::app::App as GameOfLifeApp;
use hello_quad::components::app::App as HelloQuadApp;
use hello_quad_animated::components::app::App as HelloQuadAnimatedApp;
use kaleidoscope::components::app::App as KaleidoscopeApp;
use larger_than_life::components::app::App as LargerThanLifeApp;
use ray_tracer::components::app::App as RayTracerApp;
use recording_demo::components::app::App as RecordingDemoApp;
use shared::route::Route;
use simplex_noise::components::app::App as SimplexNoiseApp;
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
        Route::SimplexNoise => html! { <SimplexNoiseApp /> },
        Route::RayTracer => html! { <RayTracerApp /> },
        Route::RecordingDemo => html! { <RecordingDemoApp /> },
        Route::Kaleidoscope => html! { <KaleidoscopeApp /> },
    }
}
