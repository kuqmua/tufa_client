use crate::constants::BACKGROUND_COLOR;
use crate::constants::DEFAULT_PADDING_PX;
use crate::constants::SHADOW_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub is_drawer_open: bool,
    pub callback: Callback<MouseEvent>,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let nav_style = format!(
        "
        transition: all 500ms cubic-bezier(0.4, 0.0, 0.2, 1);
        background-color: {};
        display: flex;
        justify-content: center;
        flex-direction: column;
        padding: {}px;
      ",
        BACKGROUND_COLOR, DEFAULT_PADDING_PX
    );
    let label_style = format!(
        "
        background-color: {}; 
      ",
        SHADOW_COLOR
    );
    let transform = if props.is_drawer_open {
        "none".to_string()
    } else {
        "translateX(-100%)".to_string()
    };
    let aside_navigation_style = format!(
        "
      position: fixed;
      z-index: 99;
      width: 350px;
      height: 100%;
      top: 0;
      bottom: 0;
      transform: {};
      display: grid;
      transition: transform 0.5s cubic-bezier(0.4, 0.0, 0.2, 1);
    ",
        transform
    );
    let aside_label_style = format!(
        "
      position: fixed;
      z-index: 98;
      width: 100%;
      height: 100%;
      top: 0;
      bottom: 0;
      transform: {};
      display: grid;
      opacity: 0.5;
  
    ",
        transform
    );
    //todo gradient color between two states
    html! {
      <>
        <aside
          style={aside_navigation_style.clone()}
        >
          <nav
            style={nav_style}
          >
            <div
              style="
                width: 100px;
                height: 100%;
                background-color: grey;
              "
            >
            </div>
          </nav>
        </aside>
        <aside
          style={aside_label_style.clone()}
        >
          <label
            for="menu-opener"
            style={label_style}
            onclick={&props.callback}
          >
          </label>
        </aside>
      </>
    }
}
