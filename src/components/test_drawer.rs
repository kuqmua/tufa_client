use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct TestDrawerProps {
    pub is_drawer_open: bool,
    pub callback: Callback<MouseEvent>,
}

#[function_component(TestDrawer)]
pub fn test_drawer(props: &TestDrawerProps) -> Html {
    let display = if props.is_drawer_open {
        "block".to_owned()
    }
    else {
        "none".to_owned()
    };
    let style_handle_section = format!(
        "
          display: {};
        ",
        display
      );
    html! {
      <>
        <section 
          class="drawer" 
        //   style={style_handle_section}
          id="drawer-section" 
        >
          <div 
            class="drawer__overlay" 
            data-drawer-close="data-drawer-close" 
            tabindex="-1"
            onclick={&props.callback}
          >
          </div>
          <div class="drawer__wrapper"></div>
        </section>
      </>
    }
}