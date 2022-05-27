use crate::components::feed::expander::component::Expander;
use crate::components::feed::posts_list::PostsList;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use yew::{function_component, html, use_state, Callback};
use crate::constants::HEADER_HEIGHT_PX;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::components::drawer::component::Drawer;
use crate::components::header::component::Header;
use crate::components::test_drawer::TestDrawer;

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
    let is_drawer_open_cloned_second = is_drawer_open;
    let oninput = Callback::from(move |_| {
        is_drawer_open_cloned_first.set(!*is_drawer_open_cloned_first);
    });
    html! {
      <>
        <Header callback={oninput.clone()}/>
        // <Drawer is_drawer_open={*is_drawer_open_cloned_second} callback={oninput}/>
        <TestDrawer/>
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
