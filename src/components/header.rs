use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    // let history = use_history().unwrap();
    // let onclick = Callback::once(move |_| history.push(Routes::Home));
    html! {
        <div style="height: 50px; width: 100%; background-color: yellow; display: flex; justify-content: space-between; align-items: center;">
            <h1>{ "Hi!" }</h1>
        </div>
    }
}
