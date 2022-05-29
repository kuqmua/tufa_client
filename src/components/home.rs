use crate::components::feed::expander::component::Expander;
use crate::components::feed::posts_list::PostsList;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use yew::{function_component, html, use_state, Callback};
use crate::constants::HEADER_HEIGHT_PX;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::components::drawer::component::Drawer;
use crate::components::header::component::Header;

#[derive(Debug, PartialEq)]
pub struct DrawerChangingStyle {
  pub display: String,
  pub transform: String,
  pub webkit_transform: String,
  pub opacity: String,
}
impl DrawerChangingStyle {
  pub fn initial() -> DrawerChangingStyle {
    DrawerChangingStyle {
      display: String::from("none"),
      transform: String::from("translate3d(-100%, 0, 0)"),
      webkit_transform: String::from("translate3d(-100%, 0, 0)"),
      opacity: String::from(""),
    }
  }
  pub fn opened_before_timeout() -> DrawerChangingStyle {
    DrawerChangingStyle {
      display: String::from("block"),
      transform: String::from("translate3d(-100%, 0, 0)"),
      webkit_transform: String::from("translate3d(-100%, 0, 0)"),
      opacity: String::from(""),
    }
  }
  pub fn opened_after_timeout() -> DrawerChangingStyle {
    DrawerChangingStyle {
      display: String::from("block"),
      transform: String::from("translate3d(0, 0, 0)"),
      webkit_transform: String::from("translate3d(0, 0, 0)"),
      opacity: String::from("0.5"),
    }
  }
  pub fn closed_before_timeout() -> DrawerChangingStyle {
    DrawerChangingStyle {
      display: String::from("block"),
      transform: String::from("translate3d(-100%, 0, 0)"),
      webkit_transform: String::from("translate3d(-100%, 0, 0)"),
      opacity: String::from(""),
    }
  }
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
    let is_drawer_open = use_state(|| false);
    let is_drawer_open_cloned_first = is_drawer_open.clone();
    let is_drawer_open_cloned_second = is_drawer_open.clone();
    let is_drawer_open_cloned_third = is_drawer_open.clone();
    let drawer_style = use_state(|| DrawerChangingStyle::initial());
    let drawer_style_cloned_first = drawer_style.clone();
    let drawer_style_cloned_second = drawer_style.clone();
    let drawer_style_cloned_third = drawer_style.clone();
    let on_open = Callback::from(move |_| {
        is_drawer_open_cloned_first.set(true);
        drawer_style_cloned_first.set(DrawerChangingStyle::opened_before_timeout());
        let drawer_style_cloned_first_another = drawer_style_cloned_first.clone();
        gloo::timers::callback::Timeout::new(50, move || {
          drawer_style_cloned_first_another.set(DrawerChangingStyle::opened_after_timeout());
      }).forget();
    });
    let on_close = Callback::from(move |_| {
      is_drawer_open_cloned_third.set(false);
      drawer_style_cloned_second.set(DrawerChangingStyle::closed_before_timeout());
      let drawer_style_cloned_second_another = drawer_style_cloned_second.clone();
      gloo::timers::callback::Timeout::new(350, move || {
        drawer_style_cloned_second_another.set(DrawerChangingStyle::initial());
      }).forget();
    });
    html! {
      <>
        <Header callback={on_open.clone()}/>
        <Drawer 
          is_drawer_open={*is_drawer_open_cloned_second} 
          callback={on_close.clone()}
          drawer_is_active_display_value={drawer_style_cloned_third.display.clone()}
          drawer_wrapper_webkit_transform={drawer_style_cloned_third.webkit_transform.clone()}
          drawer_wrapper_transform={drawer_style_cloned_third.transform.clone()}
          drawer_overlay_opacity={drawer_style_cloned_third.opacity.clone()}
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
