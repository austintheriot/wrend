use shared::route::Route;
use yew::prelude::*;
use crate::components::LinkCard;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
    <div class="home">
        <h1>
           {"Wrend Live Demos"}
        </h1>
        <div class="link-card-container">
            <LinkCard 
                route={Route::HelloQuad}
                title="Hello Quad"
                img_src="./assets/hello_quad.png"
                img_alt="Multicolored quadrilateral"
            />
            <LinkCard 
                route={Route::HelloQuadAnimated}
                title="Hello Quad Animated"
                vid_src="./assets/hello_quad_animated.webm"
            />
            <LinkCard 
                route={Route::RecordingDemo}
                title="Recording Demo"
                vid_src="./assets/hello_quad_animated.webm"
            />
            <LinkCard 
                route={Route::SimplexNoise}
                title="Simplex Noise"
                img_src="./assets/simplex_noise.png"
                img_alt="Basic 2d noise render using simplex noise"
            />
            <LinkCard 
                route={Route::GameOfLife}
                title="Game of Life"
                img_src="./assets/game_of_life.png"
                img_alt="Conway's classic game of life"
            />
            <LinkCard 
                route={Route::LargerThanLife}
                title="Larger Than Life"
                img_src="./assets/larger_than_life.png"
                img_alt="Render of the Game of Life, adjusted to a larger kernel size"
            />
            <LinkCard 
                route={Route::FlowField}
                title="Flow Field"
                img_src="./assets/flow_field.png"
                img_alt="Particles flowing across a 2D plane"
            />
            <LinkCard 
                route={Route::Kaleidoscope}
                title="Kaleidoscope"
                img_src="./assets/kaleidoscope.png"
                img_alt="Render of layered filters that make a kaleidoscope-like image"
            />
            <LinkCard 
                route={Route::RayTracer}
                title="Ray Tracer"
                img_src="./assets/ray_tracer.png"
                img_alt="Render from a custom ray tracer"
            />
       </div>
    </div>
           }
}
