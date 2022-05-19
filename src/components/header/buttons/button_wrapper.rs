use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};
use crate::constants::BACKGROUND_COLOR;

#[derive(Properties, PartialEq)]
pub struct ButtonWrapperProps {
    pub inner_html: Html,
    pub callback: Callback<MouseEvent>,
}

#[function_component(ButtonWrapper)]
pub fn button_wrapper(props: &ButtonWrapperProps) -> Html {
    let size_px: u32 = 26;
    let style_handle = format!(
      "
        width: {}px;
        height: {}px;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: {};
        border: 1px solid {};
        padding: 0px;
      ",
      size_px, size_px, BACKGROUND_COLOR, BACKGROUND_COLOR
    );
    html! {
      <button
        style={style_handle}
        onclick={&props.callback}
      >
       {props.inner_html.clone()}
      </button>
    }
}
