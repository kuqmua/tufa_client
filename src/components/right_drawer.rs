#[derive(Debug, PartialEq)]
pub enum DrawerChangingStyleState {
  Initial,
  OpenedBeforeTimeout,
  OpenedAfterTimeout,
  ClosedBeforeTimeout,
}

impl Clone for DrawerChangingStyleState {
  fn clone(&self) -> Self {
    match *self {
        DrawerChangingStyleState::Initial => DrawerChangingStyleState::Initial,
        DrawerChangingStyleState::OpenedBeforeTimeout => DrawerChangingStyleState::OpenedBeforeTimeout,
        DrawerChangingStyleState::OpenedAfterTimeout => DrawerChangingStyleState::OpenedAfterTimeout,
        DrawerChangingStyleState::ClosedBeforeTimeout => DrawerChangingStyleState::ClosedBeforeTimeout,
    }
  }
}

impl DrawerChangingStyleState {
  pub fn get_value(&self) -> RightDrawerChangingStyle {
    match *self {
        DrawerChangingStyleState::Initial => RightDrawerChangingStyle {
          display: String::from("none"),
          transform: String::from("translate3d(100%, 0, 0)"),
          webkit_transform: String::from("translate3d(100%, 0, 0)"),
          opacity: String::from(""),
        },
        DrawerChangingStyleState::OpenedBeforeTimeout => RightDrawerChangingStyle {
          display: String::from("block"),
          transform: String::from("translate3d(100%, 0, 0)"),
          webkit_transform: String::from("translate3d(100%, 0, 0)"),
          opacity: String::from(""),
        },
        DrawerChangingStyleState::OpenedAfterTimeout => RightDrawerChangingStyle {
          display: String::from("block"),
          transform: String::from("translate3d(0, 0, 0)"),
          webkit_transform: String::from("translate3d(0, 0, 0)"),
          opacity: String::from("0.5"),
        },
        DrawerChangingStyleState::ClosedBeforeTimeout => RightDrawerChangingStyle {
          display: String::from("block"),
          transform: String::from("translate3d(100%, 0, 0)"),
          webkit_transform: String::from("translate3d(100%, 0, 0)"),
          opacity: String::from(""),
        },
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct RightDrawerChangingStyle {
  pub display: String,
  pub transform: String,
  pub webkit_transform: String,
  pub opacity: String,
}

use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback};
use crate::constants::BACKGROUND_COLOR;
use crate::constants::FEED_WIDTH_PX;

#[derive(Properties, PartialEq)]
pub struct RightDrawerProps {
    pub callback: Callback<MouseEvent>,
    pub style_state: DrawerChangingStyleState,
}

#[function_component(RightDrawer)]
pub fn drawer(props: &RightDrawerProps) -> Html {
    //todo: add esc keydown handling support(from working drawer.html) 
    let changing_style = props.style_state.get_value();
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
      "
        position: fixed;
        top: 0;
        left: auto;
        right: 0;
        bottom: 0;
        height: 100%;
        width: 100%;
        width: {}px;
        z-index: 9999;
        overflow: auto;
        transition: transform 0.3s;
        will-change: transform;
        background-color: {};
        display: flex;
        flex-direction: column; 
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
            onclick={&props.callback}
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