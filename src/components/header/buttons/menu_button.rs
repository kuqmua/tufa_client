// use crate::components::header::buttons::button_wrapper::ButtonWrapper;
// use crate::components::svg::menu::Menu;
use crate::constants::HEADER_ICONS_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};
use crate::constants::BACKGROUND_COLOR;

#[derive(Properties, PartialEq)]
pub struct MenuButtonProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(MenuButton)]
pub fn menu_button(props: &MenuButtonProps) -> Html {
    let icon_size = "26px".to_owned();
    // let html_handle = html! {<Menu height={icon_size.clone()} width={icon_size} fill={HEADER_ICONS_COLOR.to_owned()}/>};
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
          onclick={props.callback.clone()}
          data-drawer-trigger="data-drawer-trigger" 
          aria-controls="drawer-name-left" 
          aria-expanded="false"
        >
          <svg 
            xmlns="http://www.w3.org/2000/svg" 
            viewBox="0 0 24 24" 
            height={icon_size.clone()} 
            width={icon_size.clone()} 
            fill={HEADER_ICONS_COLOR}
          >
            <path d="M0 0h24v24H0V0z" fill="none"/>
            <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
          </svg>   
        </button>
    }
}

// <ButtonWrapper inner_html={html_handle} callback={props.callback.clone()} id={"person_outline_button".to_owned()}/>