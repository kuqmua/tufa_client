use crate::components::yewdux_functional_display_credentials::YewduxFunctionalDisplayCredentials;
use yew::prelude::*;

#[function_component(YewduxFunctionalComponentExample)]
pub fn yewdux_functional_component_example() -> Html {
    html! {
      <div>
        <h1>{"App"}</h1>
        <YewduxFunctionalDisplayCredentials/>
      </div>
    }
}
