use crate::components::svg::logout::Logout;
use yew::{function_component, html};

#[function_component(ProfileActionsPanel)]
pub fn profile_actions_panel() -> Html {
    html! {
      <div
        style="
          position: absolute;
          top: 43px;
          right: 0px;
          height: 400px;
          width: 190px;
          border-radius: 0px 0px 0px 20px;
          border-left: 1px solid #222c36;
          border-bottom: 1px solid #222c36;
          background-color: #16202A;
          display: flex;
          justify-content: space-evenly;
          flex-direction: column;
          padding: 13px;
        "
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
