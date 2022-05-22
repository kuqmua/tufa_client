use crate::constants::BACKGROUND_COLOR;
use crate::constants::SHADOW_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub style: String,
    pub callback: Callback<MouseEvent>,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let nav_style = format!(
        "
        transition: all 500ms cubic-bezier(0.4, 0.0, 0.2, 1);
        background-color: {};
      ",
        BACKGROUND_COLOR
    );
    let label_style = format!(
        "
        background-color: {}; 
        opacity: 0.5;
      ",
        SHADOW_COLOR
    );
    html! {
      <aside
        style={props.style.clone()}
      >
        <nav
          style={nav_style}
        >
        </nav>
        <label
          for="menu-opener"
          style={label_style}
          onclick={&props.callback}
        >
        </label>
      </aside>
    }
}
