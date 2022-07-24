use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/hello-quad")]
    HelloQuad,
    #[at("/hello-quad-animated")]
    HelloQuadAnimated,
    #[at("/game-of-life")]
    GameOfLife,
    #[at("/larger-than-life")]
    LargerThanLife,
    #[at("/perlin-noise")]
    PerlinNoise,
    #[at("/simplex-noise")]
    SimplexNoise,
    #[at("/flow-field")]
    FlowField,
    #[at("/flow-field-colorful")]
    FlowFieldColorful,
    #[at("/ray-tracer")]
    RayTracer,
}
