use shared::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::LinkCard;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
    <div class="home">
        <h1>
           {"Demos"}
        </h1>
        <div class="link-card-container">
            <LinkCard 
                route={Route::HelloQuad}
                title="Hello Quad"
                img_src="./assets/hello_quad.png"
            />
            <LinkCard 
                route={Route::HelloQuadAnimated}
                title="Hello Quad Animated"
                img_src="./assets/hello_quad.png"
            />
            <LinkCard 
                route={Route::RecordingDemo}
                title="Recording Demo"
                img_src="./assets/hello_quad.png"
            />
            <LinkCard 
                route={Route::GameOfLife}
                title="Game of Life"
                img_src="./assets/hello_quad.png"
            />
            <LinkCard 
                route={Route::LargerThanLife}
                title="Larger Than Life"
                img_src="./assets/hello_quad.png"
            />
            <LinkCard 
                route={Route::SimplexNoise}
                title="Simplex Noise"
                img_src="./assets/hello_quad.png"
            />
            <LinkCard 
                route={Route::FlowField}
                title="Flow Field"
                img_src="./assets/hello_quad.png"
            />
            <LinkCard 
                route={Route::RayTracer}
                title="Ray Tracer"
                img_src="./assets/hello_quad.png"
            />
       </div>
    </div>
           }
}
