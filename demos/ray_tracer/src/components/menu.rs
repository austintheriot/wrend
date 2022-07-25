use yew::prelude::*;

use crate::state::app_context::{AppContext, AppContextError};

#[function_component(Menu)]
pub fn menu() -> Html {
    let _ = use_context::<AppContext>().expect(AppContextError::NOT_FOUND);
    
    html! {
    <div class="menu">
    </div>
    }
}
