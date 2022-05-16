use crate::components::feed::post::Post;
use yew::prelude::*;

#[function_component(PostsList)]
pub fn posts_list() -> Html {
    let posts_vec = vec![html! {<Post/>}, html! {<Post/>}, html! {<Post/>}];
    html! {
      <div
        style="
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: flex-start;
          width: 470px;
          min-width: 470px;
          padding-left: 8px;
          padding-right: 8px;
          border-right: 1px solid #1E2832;
          border-left: 1px solid #1E2832;
        "
      >
        {posts_vec}
      </div>
    }
}
