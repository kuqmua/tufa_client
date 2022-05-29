use crate::components::svg::menu::Menu;
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
          <Menu height={icon_size.clone()} width={icon_size} fill={HEADER_ICONS_COLOR.to_owned()}/> 
        </button>
    }
}