use crate::components::svg::logout::Logout;
use crate::constants::INTERFACE_LINES_COLOR;
use yew::{function_component, html};

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
    html! {
      <div
        style={style_handle}
      >
        <div
          style="
            color: white;
            display: flex;
            flex-direction: row;
            padding-top: 5px;
            padding-bottom: 5px;
            padding-left: 10px;
            padding-right: 10px;
            background-color: #ffffff33;
            border-radius: 5px 5px 5px 5px;
            height: 25px;
            display: flex;
            align-items: center;
          "
        >
          <Logout height={"22px".to_owned()} width={"22px".to_owned()} fill={"white".to_owned()}/>
          <div
            style="
              color: white;
              margin-left: 10px;
              display: flex;
              align-items: center;
            "
          >
            {"Logout"}
          </div>
        </div>
      </div>
    }
}
