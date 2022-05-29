use crate::components::feed::expander::component::Expander;
use crate::components::feed::posts_list::PostsList;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use yew::{function_component, html, use_state, Callback};
use crate::constants::HEADER_HEIGHT_PX;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::components::drawer::component::Drawer;
use crate::components::header::component::Header;
use crate::components::drawer::component::DrawerChangingStyleState;

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
    let drawer_style_cloned_on_open = drawer_style.clone();
    let on_open = Callback::from(move |_| {
      drawer_style_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_cloned_first_another = drawer_style_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
          drawer_style_cloned_first_another.set(DrawerChangingStyleState::OpenedAfterTimeout);
      }).forget();
    });
    let drawer_style_cloned_on_close = drawer_style.clone();
    let on_close = Callback::from(move |_| {
      drawer_style_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
      let drawer_style_cloned_second_another = drawer_style_cloned_on_close.clone();
      gloo::timers::callback::Timeout::new(350, move || {
        drawer_style_cloned_second_another.set(DrawerChangingStyleState::Initial);
      }).forget();
    });
    let drawer_style_enum_handle = &*drawer_style.clone();
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
