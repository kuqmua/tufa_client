use yew::{html, Callback, function_component};
use yew_router::hooks::use_history;
use crate::routes::routes::Routes;
use yew_router::prelude::*;

#[function_component(Secure)]
pub fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Routes::Home));
    let class = "my_title";
    html! {
        <div>
            if class != "my_title" {
                <h1 class={class}>{ "Secure" }</h1>
            }
            else {
                <h1 class={class}>{ "kekw" }</h1>
            }
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}