use yew::prelude::*;

#[function_component(YewduxFunctionalDisplayCredentials)]
pub fn view() -> Html {
    html! {
      <div>
        <h1>{"Display username"}</h1>
        <p>{format!("username: {}", "kekw")}</p>
        <h1>{"Display password"}</h1>
        <p>{format!("password: {}", "kekw")}</p>
      </div>
    }
}
