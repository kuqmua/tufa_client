use crate::components::feed::expander::component::Expander;
use crate::components::feed::posts_list::PostsList;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use yew::{function_component, html, use_state, Callback};
use crate::constants::HEADER_HEIGHT_PX;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::components::drawer::component::Drawer;
use crate::components::header::component::Header;

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
  pub fn get_value(&self) -> DrawerChangingStyle {
    match *self {
        DrawerChangingStyleState::Initial => DrawerChangingStyle {
          display: String::from("none"),
          transform: String::from("translate3d(-100%, 0, 0)"),
          webkit_transform: String::from("translate3d(-100%, 0, 0)"),
          opacity: String::from(""),
        },
        DrawerChangingStyleState::OpenedBeforeTimeout => DrawerChangingStyle {
          display: String::from("block"),
          transform: String::from("translate3d(-100%, 0, 0)"),
          webkit_transform: String::from("translate3d(-100%, 0, 0)"),
          opacity: String::from(""),
        },
        DrawerChangingStyleState::OpenedAfterTimeout => DrawerChangingStyle {
          display: String::from("block"),
          transform: String::from("translate3d(0, 0, 0)"),
          webkit_transform: String::from("translate3d(0, 0, 0)"),
          opacity: String::from("0.5"),
        },
        DrawerChangingStyleState::ClosedBeforeTimeout =>           DrawerChangingStyle {
          display: String::from("block"),
          transform: String::from("translate3d(-100%, 0, 0)"),
          webkit_transform: String::from("translate3d(-100%, 0, 0)"),
          opacity: String::from(""),
        },
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct DrawerChangingStyle {
  pub display: String,
  pub transform: String,
  pub webkit_transform: String,
  pub opacity: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let show_expander = use_state(|| false);
    let show_expander_cloned = show_expander.clone();
    let show_expander_cloned_second = show_expander.clone();
    let change_show_expander = Callback::from(move |_| {
        show_expander_cloned.set(!*show_expander_cloned);
    });
    let inner_html = html!{<ExpandMoreContent/>};
    let padding_summary = HEADER_HEIGHT_PX + HEADER_BORDER_BOTTOM_PX;
    let style_handle = format!(
      "
        padding-top: {}px;
      ",
      padding_summary
    );
    let drawer_style = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_cloned_first = drawer_style.clone();
    let drawer_style_cloned_second = drawer_style.clone();
    let drawer_style_enum_handle = &*drawer_style.clone().clone();
    let on_open = Callback::from(move |_| {
        drawer_style_cloned_first.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_cloned_first_another = drawer_style_cloned_first.clone();
        gloo::timers::callback::Timeout::new(50, move || {
          drawer_style_cloned_first_another.set(DrawerChangingStyleState::OpenedAfterTimeout);
      }).forget();
    });
    let on_close = Callback::from(move |_| {
      drawer_style_cloned_second.set(DrawerChangingStyleState::ClosedBeforeTimeout);
      let drawer_style_cloned_second_another = drawer_style_cloned_second.clone();
      gloo::timers::callback::Timeout::new(350, move || {
        drawer_style_cloned_second_another.set(DrawerChangingStyleState::Initial);
      }).forget();
    });
    html! {
      <>
        <Header callback={on_open.clone()}/>
        <Drawer 
          callback={on_close.clone()}
          style_state={drawer_style_enum_handle.clone()}
        />
        <div
          style="
            width: 100%; 
            height: 100%;
            display: flex; 
            justify-content: center; 
            flex-direction: column; 
            align-items: center;
          "
        >
          <div
            style={style_handle}
          >
            <PostsList callback={change_show_expander} is_expander_opened={*show_expander_cloned_second.clone()}/>
            if *show_expander {
              <Expander inner_html={inner_html}/>
            }
          </div>
        </div>
      </>
    }
}
