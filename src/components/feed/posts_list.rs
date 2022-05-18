use crate::components::feed::post::Post;
use crate::constants::INTERFACE_LINES_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct PostsListProps {
    pub callback: Callback<MouseEvent>,
    pub is_expander_opened: bool,
}

#[function_component(PostsList)]
pub fn posts_list(props: &PostsListProps) -> Html {
    let posts_vec = vec![
        html! {<Post callback={props.callback.clone()}/>},
        html! {<Post callback={props.callback.clone()}/>},
        html! {<Post callback={props.callback.clone()}/>},
    ];
    let style_handle = format!(
        "
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: flex-start;
          width: 470px;
          min-width: 470px;
          padding-left: 8px;
          padding-right: 8px;
          border-right: 1px solid {};
          border-left: 1px solid {};
        ",
        INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR
    );
    html! {
      <div
        style={style_handle}
      >
        {posts_vec}
      </div>
    }
}
