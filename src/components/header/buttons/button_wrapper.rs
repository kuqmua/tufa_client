use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonWrapperProps {
    pub inner_html: Html,
    pub callback: Callback<MouseEvent>,
}

#[function_component(ButtonWrapper)]
pub fn button_wrapper(props: &ButtonWrapperProps) -> Html {
    html! {
      <button
        style="
          width: 26px;
          height: 26px;
          display: flex;
          justify-content: center;
          align-items: center;
          background-color: #16202a;
          border: 1px solid #16202a;
          padding: 0px;
        "
        onclick={&props.callback}
      >
       {props.inner_html.clone()}
      </button>
    }
}
