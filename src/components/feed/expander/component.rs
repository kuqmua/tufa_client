use crate::components::feed::expander::touch_line::TouchLine;
use crate::constants::BACKGROUND_COLOR;
use crate::constants::FEED_WIDTH_PX;
use crate::constants::INTERFACE_LINES_COLOR;
use web_sys::MouseEvent;
use yew::Callback;
use yew::{function_component, html, Html, Properties};

use crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState;

// #[derive(Properties, PartialEq)]
// pub struct ExpanderProps {
//     pub inner_html: Html,
// }

// #[function_component(Expander)]
// pub fn expander(props: &ExpanderProps) -> Html {
//     let border_radius = "30px";
//     let style_handle = format!(
//         "
//         height: fit-content; 
//         width: {}px;
//         min-width: {}px;
//         background-color: {};
//         border-top: 1px solid {};
//         border-left: 1px solid {};
//         border-right: 1px solid {};
//         position: fixed;
//         border-radius: {} {} 0px 0px;
//         bottom: 0px;
//         display: flex;
//         align-items: center;
//         flex-direction: column;
//         padding: 8px;
//       ",
//         FEED_WIDTH_PX,
//         FEED_WIDTH_PX,
//         BACKGROUND_COLOR,
//         INTERFACE_LINES_COLOR,
//         INTERFACE_LINES_COLOR,
//         INTERFACE_LINES_COLOR,
//         border_radius,
//         border_radius
//     );
//     html! {
//       <div
//         style={style_handle}
//       >
//           <TouchLine/>
//           {props.inner_html.clone()}
//       </div>
//     }
// }

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ExpanderProps {
    // pub callback: Callback<MouseEvent>,
    pub style_state: ExpanderChangingStyleState,
    pub inner_html: Html,
}

#[function_component(Expander)]
pub fn expander(props: &ExpanderProps) -> Html {
    //todo: add esc keydown handling support(from working drawer.html)
    let changing_style = &props.style_state.get_value();
    let section_style = format!(
        "
        display: {};
      ",
        changing_style.display
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
        changing_style.opacity
    );
    let drawer_wrapper_style = format!(
      // left: 0;
      // right: 0;
        "
        position: fixed;
        top: auto;
        bottom: 0;
        height: 400px;
        width: 100%;
        width: {}px;
        z-index: 9999;
        overflow: auto;
        transition: transform 0.3s;
        will-change: transform;
        background-color: {};
        display: flex;
        flex-direction: column; 
        justify-content: flex-start;
        align-items: center;
        -webkit-overflow-scrolling: touch; /* enables momentum scrolling in iOS overflow elements */
        -webkit-transform: {};
        transform: {};
      ",
        FEED_WIDTH_PX,
        BACKGROUND_COLOR,
        changing_style.webkit_transform,
        changing_style.transform
    );
    html! {
      <>
        <section
          style={section_style}
        >
          <div
            style={drawer_overlay_style}
            // onclick={&props.callback.clone()}
          >
          </div>
          <div
            style={drawer_wrapper_style}
          >
          </div>
        </section>
      </>
    }
}

