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
          border-radius: 10px;
          border: 1px solid #222c36;
          width: 35px;
          height: 35px;
          margin-bottom: 8px;
          display: flex;
          justify-content: center;
          align-items: center;
          background-color: #222c36;
        "
        onclick={&props.callback}
      >
       {props.inner_html.clone()}
      </button>
    }
}
