use crate::components::drawer::component::Drawer;
use crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState;
use crate::components::drawer::position::DrawerPosition;
use crate::components::feed::expander::component::Expander;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use crate::components::feed::expander::share_content::ShareContent;
use crate::components::feed::posts_list::PostsList;
use crate::components::header::component::Header;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::constants::HEADER_HEIGHT_PX;
use yew::{function_component, html, use_state, Callback};

use crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState;

#[derive(Debug, PartialEq)]
pub enum ExpanderStatus {
    Closed,
    Share,
    ExpandMore,
}

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

    //
    let drawer_style_left = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_left_cloned_on_open = drawer_style_left.clone();
    let expander_status_clone_drawer_on_open_left = expander_status.clone();
    let on_open_left = Callback::from(move |_| {
        expander_status_clone_drawer_on_open_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_left_cloned_first_another = drawer_style_left_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_left_cloned_first_another
                .set(DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_left_cloned_on_close = drawer_style_left.clone();
    let expander_status_clone_drawer_on_close_left = expander_status.clone();
    let on_close_left = Callback::from(move |_| {
        expander_status_clone_drawer_on_close_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_left_cloned_second_another = drawer_style_left_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_left_cloned_second_another.set(DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_left_enum_handle = &*drawer_style_left.clone();
    ////
    let drawer_style_right = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_right_cloned_on_open = drawer_style_right.clone();
    let expander_status_clone_drawer_on_open_right = expander_status.clone();
    let on_open_right = Callback::from(move |_| {
        expander_status_clone_drawer_on_open_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_right_cloned_first_another = drawer_style_right_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_right_cloned_first_another
                .set(DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_right_cloned_on_close = drawer_style_right.clone();
    let expander_status_clone_drawer_on_close_right = expander_status.clone();
    let on_close_right = Callback::from(move |_| {
        expander_status_clone_drawer_on_close_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_right_cloned_second_another = drawer_style_right_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_right_cloned_second_another.set(DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_right_enum_handle = &*drawer_style_right.clone();
    let expander_status_cloned_share = expander_status.clone();
    // let expander_status_to_share = Callback::from(move |_| match *expander_status_cloned_share {
    //     ExpanderStatus::Share => {
    //         expander_status_cloned_share.set(ExpanderStatus::Closed);
    //     }
    //     _ => {
    //         expander_status_cloned_share.set(ExpanderStatus::Share);
    //     }
    // });
    let expander_status_cloned_expand_more = expander_status.clone();
    // let expander_status_to_expand_more =
    //     Callback::from(move |_| match *expander_status_cloned_expand_more {
    //         ExpanderStatus::ExpandMore => {
    //             expander_status_cloned_expand_more.set(ExpanderStatus::Closed);
    //         }
    //         _ => {
    //             expander_status_cloned_expand_more.set(ExpanderStatus::ExpandMore);
    //         }
    //     });
    let share_inner_html = html! {<ShareContent/>};
    let expand_more_inner_html = html! {<ExpandMoreContent/>};
    let inner_html_left = html! {};
    let inner_html_right = html! {};
    //
    //
    //
    let expander_style = use_state(|| ExpanderChangingStyleState::Initial);
    let expander_style_clone_open_expand_more = expander_style.clone();
    // let expander_status_clone_drawer_on_open_left = expander_status.clone();
    let expander_on_open_expand_more = Callback::from(move |_| {
        // expander_status_clone_drawer_on_open_left.set(ExpanderStatus::Closed);

        //
        match *expander_status_cloned_expand_more {
                  ExpanderStatus::ExpandMore => {
                      expander_status_cloned_expand_more.set(ExpanderStatus::Closed);
                  }
                  _ => {
                      expander_status_cloned_expand_more.set(ExpanderStatus::ExpandMore);
                  }
              }
        //
        expander_style_clone_open_expand_more.set(ExpanderChangingStyleState::OpenedBeforeTimeout);
        let expander_style_clone_open_another = expander_style_clone_open_expand_more.clone();
        gloo::timers::callback::Timeout::new(50, move || {
          expander_style_clone_open_another
                .set(ExpanderChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let expander_style_clone_open_share = expander_style.clone();
    let expander_on_open_share = Callback::from(move |_| {
      // expander_status_clone_drawer_on_open_left.set(ExpanderStatus::Closed);

      //
      match *expander_status_cloned_share {
        ExpanderStatus::Share => {
            expander_status_cloned_share.set(ExpanderStatus::Closed);
        }
        _ => {
            expander_status_cloned_share.set(ExpanderStatus::Share);
        }
    }
      //
      expander_style_clone_open_share.set(ExpanderChangingStyleState::OpenedBeforeTimeout);
      let expander_style_clone_open_another = expander_style_clone_open_share.clone();
      gloo::timers::callback::Timeout::new(50, move || {
        expander_style_clone_open_another
              .set(ExpanderChangingStyleState::OpenedAfterTimeout);
      })
      .forget();
  });
    let expander_style_clone_close = expander_style.clone();
    // let expander_status_clone_drawer_on_close_left = expander_status.clone();
    let expander_on_close = Callback::from(move |_| {
        // expander_status_clone_drawer_on_close_left.set(ExpanderStatus::Closed);
        expander_style_clone_close.set(ExpanderChangingStyleState::ClosedBeforeTimeout);
        let expander_style_clone_close_another = expander_style_clone_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            expander_style_clone_close_another.set(ExpanderChangingStyleState::Initial);
        })
        .forget();
    });
    let expander_style_clone_close_handle = &*expander_style.clone().clone();
    let expander_handler = match *expander_status_clone_for_logic {
      ExpanderStatus::Closed => html! {}, //maybe rewrite it somehow?
      ExpanderStatus::Share => html! {
        <Expander 
          callback={expander_on_close}
          style_state={expander_style_clone_close_handle.clone()}
          inner_html={share_inner_html}
        />
      },
      ExpanderStatus::ExpandMore => html! {
        <Expander 
          callback={expander_on_close}
          style_state={expander_style_clone_close_handle.clone()}
          inner_html={expand_more_inner_html}
        />
      },
  };
    html! {
      <>
        <Header left_drawer_callback={on_open_left.clone()} right_drawer_callback={on_open_right.clone()}/>
        <Drawer
          callback={on_close_left.clone()}
          style_state={drawer_style_left_enum_handle.clone()}
          drawer_position={DrawerPosition::Left}
          inner_html={inner_html_left}
        />
        <Drawer
          callback={on_close_right.clone()}
          style_state={drawer_style_right_enum_handle.clone()}
          drawer_position={DrawerPosition::Right}
          inner_html={inner_html_right}
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
            <PostsList
              share_callback={expander_on_open_share.clone()}//expander_status_to_share
              expand_more_callback={expander_on_open_expand_more.clone()}//expander_status_to_expand_more
            />
            {expander_handler}
          </div>
        </div>
      </>
    }
}
