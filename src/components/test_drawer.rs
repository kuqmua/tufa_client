use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct TestDrawerProps {
    pub is_drawer_open: bool,
    pub callback: Callback<MouseEvent>,
    pub drawer_is_active_display_value: String,
    pub drawer_wrapper_webkit_transform: String,
    pub drawer_wrapper_transform: String,
    pub drawer_overlay_opacity: String,
}

#[function_component(TestDrawer)]
pub fn test_drawer(props: &TestDrawerProps) -> Html {
    let section_style = format!(
        "
          display: {};
        ",
        props.drawer_is_active_display_value.clone()
      );
      let drawer_overlay_style = format!(
        "
          position: fixed;
          top: 0;
          right: 0;
          bottom: 0;
          left: 0;
          width: 100%;
          z-index: 200;
          opacity: 0;
          transition: opacity 0.3s;
          will-change: opacity;
          background-color: #000;
          -webkit-user-select: none;
          -moz-user-select: none;
          -ms-user-select: none;
          user-select: none; 
          opacity: {};
        ",
        props.drawer_overlay_opacity.clone()
      );
      let drawer_wrapper_style = format!(
        "
         -webkit-transform: {};//translate3d(0, 0, 0)
         transform: {};//translate3d(0, 0, 0)
        ",
        props.drawer_wrapper_webkit_transform.clone(),
        props.drawer_wrapper_transform.clone()
      );
    html! {
      <>
        <section 
          class="drawer" 
          style={section_style}
          id="drawer-section" 
        >
          <div 
            data-drawer-close="data-drawer-close" 
            style={drawer_overlay_style}
            onclick={&props.callback}
          >
          </div>
          <div 
            class="drawer__wrapper"
            style={drawer_wrapper_style}
          >
          </div>
        </section>
      </>
    }
}