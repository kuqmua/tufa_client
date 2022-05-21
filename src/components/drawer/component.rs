use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub style: String,
    pub callback: Callback<MouseEvent>,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    html! {
      <aside 
        style={props.style.clone()}
      >
        <nav 
          style="
            transition: all 500ms cubic-bezier(0.4, 0.0, 0.2, 1);
            background-color: rgb(21, 255, 0);
          "
        >
        </nav>
        <label 
          for="menu-opener" 
          style="
            background-color: yellow; 
            opacity: 0.5;
          "
          onclick={&props.callback}
        >
        </label>
      </aside>
    }
}
