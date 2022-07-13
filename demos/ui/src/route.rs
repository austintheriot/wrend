use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/hello_quad")]
    HelloQuad,
    #[at("/hello_quad_animated")]
    HelloQuadAnimated,
    #[at("/game_of_life")]
    GameOfLife,
}