use yew::{html, Callback, function_component};
use yew_router::hooks::use_history;
use crate::routes::routes::Routes;
use yew_router::prelude::*;

#[function_component(Secure)]
pub fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Routes::Home));
    let class = "my_title";
    let option: Option<&str> = Some("kekw");
    let example_list = vec!["one", "two", "three"];
    let example_list_html = vec![html!{ <div>{"one"}</div>}, html!{ <div>{"two"}</div>}, html!{ <div>{"three"}</div>}];
    html! {
        <div>
            if class == "my_title" {
                <h1 class={class}>{ "my_title" }</h1>
            }
            else {
                <h1 class={class}>{ "not my_title" }</h1>
            }
            if let Some(option) = option {
                <h1 class={class}>{option}</h1>
            } 
            else  {
                <h1 class={class}>{"None"}</h1>
            }
            {example_list}
            {example_list_html}
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}