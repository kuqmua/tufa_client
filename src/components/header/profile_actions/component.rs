use crate::routes::routes::Routes;
use crate::constants::INTERFACE_LINES_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback};
use yew_router::history::History;
use yew_router::hooks::use_history;
use crate::components::header::profile_actions::buttons::logout_button::LogoutButton;

#[function_component(ProfileActions)]
pub fn profile_actions() -> Html {
    let style_handle = format!(
        "
        position: absolute;
        top: 43px;
        right: 0px;
        height: 400px;
        width: 190px;
        border-radius: 0px 0px 0px 20px;
        border-left: 1px solid {};
        border-bottom: 1px solid {};
        background-color: #16202A;
        display: flex;
        justify-content: space-evenly;
        flex-direction: column;
        padding: 13px;
      ",
        INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR
    );
    let history = use_history().unwrap();
    let go_to_sign_in: Callback<MouseEvent> = Callback::once(move |_| {
      history.push(Routes::SignIn);
    });
    html! {
      <div
        style={style_handle}
      >
        <LogoutButton callback={go_to_sign_in}/>
      </div>
    }
}
