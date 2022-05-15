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
          background-color: #1E2832;
          padding-left: 8px;
          padding-right: 8px;
          border-right: 0.5px solid #ffffa2;
          border-left: 0.5px solid #ffffa2;
        "
      >
        {posts_vec}
      </div>
    }
}
