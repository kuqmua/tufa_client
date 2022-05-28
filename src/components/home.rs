use crate::components::feed::expander::component::Expander;
use crate::components::feed::posts_list::PostsList;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use yew::{function_component, html, use_state, Callback};
use crate::constants::HEADER_HEIGHT_PX;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
// use crate::components::drawer::component::Drawer;
use crate::components::header::component::Header;
use crate::components::test_drawer::TestDrawer;
use gloo::console::log;

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
    let on_open = Callback::from(move |_| {
        log!("before is_drawer_open", *is_drawer_open_cloned_first.clone());
        is_drawer_open_cloned_first.set(true);
        log!("after is_drawer_open", *is_drawer_open_cloned_first.clone());
    });
    let on_close = Callback::from(move |_| {
      log!("before is_drawer_open", *is_drawer_open_cloned_third.clone());
      is_drawer_open_cloned_third.set(false);
      log!("after is_drawer_open", *is_drawer_open_cloned_third.clone());
  });
    html! {
      <>
        //
        <style>
          {"
          .drawer {
            display: none;
          }
          .drawer__overlay {
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
          }
          .drawer__header {
            padding: 1.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid #ddd;
          }
          .drawer__close {
            margin: 0;
            padding: 0;
            border: none;
            background-color: transparent;
            cursor: pointer;
            width: 15px;
            height: 15px;
            flex-shrink: 0;
            margin-left: 1rem;
          }
          .drawer__wrapper {
            position: fixed;
            top: 0;
            right: 0;
            bottom: 0;
            height: 100%;
            width: 100%;
            max-width: 500px;
            z-index: 9999;
            overflow: auto;
            transition: transform 0.3s;
            will-change: transform;
            background-color: #fff;
            display: flex;
            flex-direction: column; 
            -webkit-transform: translate3d(103%, 0, 0);
            transform: translate3d(103%, 0, 0); /* extra 3% because of box-shadow */ 
            -webkit-overflow-scrolling: touch; /* enables momentum scrolling in iOS overflow elements */
            box-shadow: 0 2px 6px #777;
          }
          .drawer__wrapper {
            left: 0;
            right: auto;
            -webkit-transform: translate3d(-100%, 0, 0);
            transform: translate3d(-100%, 0, 0);
          }
          .drawer.is-active {
            display: block;
          }
          .drawer.is-visible .drawer__wrapper {
            -webkit-transform: translate3d(0, 0, 0);
            transform: translate3d(0, 0, 0);
          }
          .drawer.is-visible .drawer__overlay {
            opacity: 0.5;
          }
          "}
        </style>
        //
        <Header callback={on_open.clone()}/>
        // <Drawer is_drawer_open={*is_drawer_open_cloned_second} callback={oninput}/>
        <TestDrawer is_drawer_open={*is_drawer_open_cloned_second} callback={on_close.clone()}/>
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
