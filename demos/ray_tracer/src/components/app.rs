use crate::components::canvas::Canvas;
use crate::components::keyboard_listener::KeyboardListener;
use crate::components::global_listeners::GlobalListeners;
use crate::components::menu::Menu;
use crate::state::app_context::AppContext;
use ui::route::Route;
use yew::{function_component, html, prelude::*};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ContextProvider<AppContext> context={AppContext::default()}>
            <KeyboardListener>
                <GlobalListeners>
                    <div class="ray-tracer">
                        <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                        <Canvas />
                        <Menu />
                    </div>
                </ GlobalListeners>
            </ KeyboardListener>
        </ContextProvider<AppContext>>

    }
}
