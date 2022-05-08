use crate::components::authorization::display_credentials::DisplayCredentials;
use yew::prelude::*;

#[function_component(YewduxFunctionalComponentExample)]
pub fn yewdux_functional_component_example() -> Html {
    html! {
      <div>
        <h1>{"App"}</h1>
        <DisplayCredentials/>
      </div>
    }
}
