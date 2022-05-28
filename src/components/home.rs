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
    
    let drawer_is_active_display_value = use_state(|| String::from("none"));
    let drawer_is_active_display_value_cloned_first = drawer_is_active_display_value.clone();
    let drawer_is_active_display_value_cloned_second = drawer_is_active_display_value.clone();
    let drawer_is_active_display_value_cloned_third = drawer_is_active_display_value.clone();
    let display_value = &*drawer_is_active_display_value_cloned_third.clone();

    let drawer_wrapper_webkit_transform_value = use_state(|| String::from("translate3d(-100%, 0, 0)"));
    let drawer_wrapper_webkit_transform_value_cloned_first = drawer_wrapper_webkit_transform_value.clone();
    let drawer_wrapper_webkit_transform_value_cloned_second = drawer_wrapper_webkit_transform_value.clone();
    let drawer_wrapper_webkit_transform_value_cloned_third = drawer_wrapper_webkit_transform_value.clone();
    let drawer_wrapper_webkit_transform = &*drawer_wrapper_webkit_transform_value_cloned_third.clone();

    let drawer_wrapper_transform_value = use_state(|| String::from("translate3d(-100%, 0, 0)"));
    let drawer_wrapper_transform_value_cloned_first = drawer_wrapper_transform_value.clone();
    let drawer_wrapper_transform_value_cloned_second = drawer_wrapper_transform_value.clone();
    let drawer_wrapper_transform_value_cloned_third = drawer_wrapper_transform_value.clone();
    let drawer_wrapper_transform = &*drawer_wrapper_transform_value_cloned_third.clone();

    let drawer_overlay_opacity_value = use_state(|| String::from(""));
    let drawer_overlay_opacity_value_cloned_first = drawer_overlay_opacity_value.clone();
    let drawer_overlay_opacity_value_cloned_second = drawer_overlay_opacity_value.clone();
    let drawer_overlay_opacity_value_cloned_third = drawer_overlay_opacity_value.clone();
    let drawer_overlay_opacity = &*drawer_overlay_opacity_value_cloned_third.clone();
    
    //-webkit-transform: {};//translate3d(0, 0, 0)
    //transform: {};//translate3d(0, 0, 0)
    //opacity: {};//0.5

    let on_open = Callback::from(move |_| {
        log!("before is_drawer_open", *is_drawer_open_cloned_first.clone());
        is_drawer_open_cloned_first.set(true);
        log!("after is_drawer_open", *is_drawer_open_cloned_first.clone());
        drawer_is_active_display_value_cloned_first.set(String::from("block"));
        let drawer_wrapper_webkit_transform_value_cloned_first_cloned = drawer_wrapper_webkit_transform_value_cloned_first.clone();
        let drawer_wrapper_transform_value_cloned_first_cloned = drawer_wrapper_transform_value_cloned_first.clone();
        let drawer_overlay_opacity_value_cloned_first_cloned = drawer_overlay_opacity_value_cloned_first.clone();
        gloo::timers::callback::Timeout::new(50, move || {
          drawer_wrapper_webkit_transform_value_cloned_first_cloned.set(String::from("translate3d(0, 0, 0)"));
          drawer_wrapper_transform_value_cloned_first_cloned.set(String::from("translate3d(0, 0, 0)"));
          drawer_overlay_opacity_value_cloned_first_cloned.set(String::from("0.5"));
      }).forget();
    });
    let on_close = Callback::from(move |_| {
      log!("before is_drawer_open", *is_drawer_open_cloned_third.clone());
      is_drawer_open_cloned_third.set(false);
      log!("after is_drawer_open", *is_drawer_open_cloned_third.clone());
      drawer_wrapper_webkit_transform_value_cloned_second.set(String::from("translate3d(-100%, 0, 0)"));
      drawer_wrapper_transform_value_cloned_second.set(String::from("translate3d(-100%, 0, 0)"));
      drawer_overlay_opacity_value_cloned_second.set(String::from(""));
      let drawer_is_active_display_value_cloned_second_cloned = drawer_is_active_display_value_cloned_second.clone();
      gloo::timers::callback::Timeout::new(350, move || {
        drawer_is_active_display_value_cloned_second_cloned.set(String::from("none"));
    }).forget();
  });
  
  //


  //
    html! {
      <>
        <Header callback={on_open.clone()}/>
        // <Drawer is_drawer_open={*is_drawer_open_cloned_second} callback={oninput}/>
        <TestDrawer 
          is_drawer_open={*is_drawer_open_cloned_second} 
          callback={on_close.clone()}
          drawer_is_active_display_value={display_value.clone()}
          drawer_wrapper_webkit_transform={drawer_wrapper_webkit_transform.clone()}
          drawer_wrapper_transform={drawer_wrapper_transform.clone()}
          drawer_overlay_opacity={drawer_overlay_opacity.clone()}
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
