use yew::{html, Callback, function_component};
// use yew_router::hooks::use_history;
// use crate::routes::routes::Routes;
// use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    // let history = use_history().unwrap();

    // let onclick = Callback::once(move |_| history.push(Routes::Home));
    html! {
        <div style="height: 50px; width: 100%; background-color: yellow; display: flex; justify-content: center; align-items: center;">
            <h1>{ "Hi!" }</h1>
        </div>
    }
}