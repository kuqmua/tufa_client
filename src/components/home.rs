use crate::components::feed::expander::component::Expander;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use crate::components::feed::expander::share_content::ShareContent;
use crate::components::feed::posts_list::PostsList;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::constants::HEADER_HEIGHT_PX;
use yew::{function_component, html, use_state, Callback};
use crate::components::drawer::component::Drawer;
use crate::components::header::component::Header;
use crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState;
use crate::components::drawer::position::DrawerPosition;

#[derive(Debug, PartialEq)]
pub enum ExpanderStatus {
    Closed,
    Share,
    ExpandMore,
}

// impl Clone for ExpanderStatus {
//   fn clone(&self) -> Self {
//     match *self {
//         ExpanderStatus::Closed => ExpanderStatus::Closed,
//         ExpanderStatus::Share => ExpanderStatus::Share,
//         ExpanderStatus::ExpandMore => ExpanderStatus::ExpandMore,
//     }
//   }
// }

#[function_component(Home)]
pub fn home() -> Html {
    let padding_summary = HEADER_HEIGHT_PX + HEADER_BORDER_BOTTOM_PX;
    let style_handle = format!(
        "
        padding-top: {}px;
      ",
        padding_summary
    );
    let expander_status = use_state(|| ExpanderStatus::Closed);
    let expander_status_clone_for_logic = expander_status.clone();

    let drawer_style = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_cloned_on_open = drawer_style.clone();
    let expander_status_clone_drawer_on_open = expander_status.clone();
    let on_open = Callback::from(move |_| {
        expander_status_clone_drawer_on_open.set(ExpanderStatus::Closed);
        drawer_style_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_cloned_first_another = drawer_style_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_cloned_first_another.set(DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_cloned_on_close = drawer_style.clone();
    let expander_status_clone_drawer_on_close = expander_status.clone();
    let on_close = Callback::from(move |_| {
        expander_status_clone_drawer_on_close.set(ExpanderStatus::Closed);
        drawer_style_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_cloned_second_another = drawer_style_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_cloned_second_another.set(DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_enum_handle_left = &*drawer_style.clone();
    let drawer_style_enum_handle_right = &*drawer_style.clone();
    let expander_status_cloned_share = expander_status.clone();
    let expander_status_to_share = Callback::from(move |_| match *expander_status_cloned_share {
        ExpanderStatus::Share => {
            expander_status_cloned_share.set(ExpanderStatus::Closed);
        }
        _ => {
            expander_status_cloned_share.set(ExpanderStatus::Share);
        }
    });
    let expander_status_cloned_expand_more = expander_status.clone();
    let expander_status_to_expand_more =
        Callback::from(move |_| match *expander_status_cloned_expand_more {
            ExpanderStatus::ExpandMore => {
                expander_status_cloned_expand_more.set(ExpanderStatus::Closed);
            }
            _ => {
                expander_status_cloned_expand_more.set(ExpanderStatus::ExpandMore);
            }
        });
    let share_inner_html = html! {<ShareContent/>};
    let expand_more_inner_html = html! {<ExpandMoreContent/>};
    let expander_handler = match *expander_status_clone_for_logic {
        ExpanderStatus::Closed => html! {}, //maybe rewrite it somehow?
        ExpanderStatus::Share => html! {<Expander inner_html={share_inner_html}/>},
        ExpanderStatus::ExpandMore => html! {<Expander inner_html={expand_more_inner_html}/>},
    };

    html! {
      <>
        <Header callback={on_open.clone()}/>
        <Drawer
          callback={on_close.clone()}
          style_state={drawer_style_enum_handle_left.clone()}
          drawer_position={DrawerPosition::Left}
        />
        <Drawer
          callback={on_close.clone()}
          style_state={drawer_style_enum_handle_right.clone()}
          drawer_position={DrawerPosition::Right}
        />
        // <RightDrawer
        //   callback={on_close.clone()}
        //   style_state={drawer_style_enum_handle.clone()}
        // />
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
            <PostsList
              share_callback={expander_status_to_share}
              expand_more_callback={expander_status_to_expand_more}
            />
            {expander_handler}
          </div>
        </div>
      </>
    }
}
