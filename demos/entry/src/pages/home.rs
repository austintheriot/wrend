use shared::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
    <div class="home">
       <h1>
           {"Demos"}
       </h1>
       <Link<Route>
           to={Route::HelloQuad}
       >
           {"Hello Quad"}
       </Link<Route>>
       <Link<Route>
           to={Route::HelloQuadAnimated}
       >
           {"Hello Quad Animated"}
       </Link<Route>>
       <Link<Route>
           to={Route::GameOfLife}
       >
           {"Game of Life"}
       </Link<Route>>
       <Link<Route>
           to={Route::LargerThanLife}
       >
           {"Larger Than Life"}
       </Link<Route>>
       <Link<Route>
           to={Route::SimplexNoise}
       >
           {"Simplex Noise"}
       </Link<Route>>
       <Link<Route>
           to={Route::FlowField}
       >
           {"Particle Flow Field"}
       </Link<Route>>
       <Link<Route>
           to={Route::FlowFieldColorful}
       >
           {"Particle Flow Field Colorful"}
       </Link<Route>>
       <Link<Route>
       to={Route::RayTracer}
       >
           {"Ray Tracer"}
       </Link<Route>>
       <Link<Route>
           to={Route::RecordingDemo}
       >
           {"Recording Demo"}
       </Link<Route>>
    </div>
           }
}
