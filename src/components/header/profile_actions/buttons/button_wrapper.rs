use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};
// use crate::constants::BACKGROUND_COLOR;

#[derive(Properties, PartialEq)]
pub struct ButtonWrapperProps {
    pub inner_html: Html,
    pub callback: Callback<MouseEvent>,
    pub placeholder: String
}

#[function_component(ButtonWrapper)]
pub fn button_wrapper(props: &ButtonWrapperProps) -> Html {
    let border_radius_px: u32 = 5;
    let style_handle = format!(
      "
        color: white;
        display: flex;
        flex-direction: row;
        padding-top: 5px;
        padding-bottom: 5px;
        padding-left: 10px;
        padding-right: 10px;
        background-color: #ffffff33;
        border-radius: {}px {}px {}px {}px;
        height: 35px;
        display: flex;
        align-items: center;
        border: 1px solid #ffffff33;
      ",
      border_radius_px, border_radius_px, border_radius_px, border_radius_px
    );
    html! {
      <button
        style={style_handle}
        onclick={&props.callback}
      >
        {props.inner_html.clone()}
        <div
          style="
            color: white;
            margin-left: 10px;
            display: flex;
            align-items: center;
            font-family: Inter;
          "
        >
          {props.placeholder.clone()}
        </div>
      </button>
    }
}