use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
     <div>
         {"404 Not Found"}
     </div>
    }
}
