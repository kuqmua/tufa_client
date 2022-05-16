use crate::components::feed::posts_list::PostsList;
use crate::components::feed::expander::Expander;
use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <div
      style="
        padding-top: 50px;
      "
    >
      <PostsList/>
      <Expander/>
    </div>
  }
}