use yew::prelude::*;
use crate::components::feed::post::Post;

#[function_component(PostsList)]
pub fn posts_list() -> Html {
    let posts_vec= vec![
        html!{<Post/>},
        html!{<Post/>},
        html!{<Post/>}
    ];
    html! {
      <div
        style="
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: flex-start;
          width: 500px;
          background-color: #1E2832;
          padding-left: 8px;
          padding-right: 8px;
        "
      >
        {posts_vec}
      </div>
    }
}