use crate::components::header::buttons::person_outline_button::PersonOutlineButton;
use crate::components::header::profile_actions_panel::ProfileActionsPanel;
use crate::components::svg::menu::Menu;
use crate::constants::HEADER_ICONS_COLOR;
use web_sys::MouseEvent;
// use crate::routes::routes::Routes;
use yew::{function_component, html, use_state, Callback};
// use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    let profile_actions_panel_opened = use_state(|| false);
    let profile_actions_panel_opened_cloned = profile_actions_panel_opened.clone();
    let change_profile_actions_panel_opened: Callback<MouseEvent> = Callback::from(move |_| {
        profile_actions_panel_opened_cloned.set(!*profile_actions_panel_opened_cloned);
    });
    html! {
      <header
        style="
          width: 100%;
          min-width: 250px;
          background-color: #16202A;
          position: fixed;
          display: flex;
          flex-direction: column;
        ">
          <div
            style="
              height: 42px; 
              border-bottom: 1px solid #222c36;
              display: flex;
              flex-direction: column;
            "
          >
            <div
              style="
                display: flex;
                height: 100%; 
                justify-content: space-between; 
                align-items: center;
                padding-right: 20px;
                padding-left: 20px;
              "
            >
              <div
                style="
                  display: flex;
                  justify-content: center;
                  align-items: center;
                "
              >
                <Menu height={"26px".to_owned()} width={"26px".to_owned()} fill={HEADER_ICONS_COLOR.to_owned()}/>
              </div>
              <div
                style="
                  font-size: 30px;
                  font-family: 'Koulen', cursive;
                  color: white;
                "
              >
                {"Tufa Client"}
              </div>
          // //   <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
          // //     {"----------"}
          // //   <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
              <PersonOutlineButton callback={change_profile_actions_panel_opened}/>
            </div>
          </div>
          if *profile_actions_panel_opened {
            <ProfileActionsPanel/>
          }
      </header>
    }
}
