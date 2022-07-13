use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello_quad")]
    HelloQuad,
    #[at("/hello_quad_animated")]
    HelloQuadAnimated,
    #[not_found]
    #[at("/404")]
    NotFound,
}