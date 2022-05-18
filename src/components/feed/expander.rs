use crate::constants::INTERFACE_LINES_COLOR;
use yew::{function_component, html};

#[function_component(Expander)]
pub fn expander() -> Html {
    let style_handle = format!(
        "
        height: 400px; 
        width: 486px;
        min-width: 470px;
        background-color: #16202A;
        border-top: 1px solid {};
        border-left: 1px solid {};
        border-right: 1px solid {};
        position: fixed;
        border-radius: 30px 30px 0px 0px;
        bottom: 0px;
        display: flex;
        justify-content: space-evenly;
        align-items: center;
        flex-direction: column;
      ",
        INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR
    );
    html! {
      <div
        style={style_handle}
      >
          <div
            style="
              color: white
          ">
            {"one"}
          </div>
          <div
            style="
              color: white
          ">
            {"two"}
          </div>
          <div
            style="
              color: white
          ">
            {"three"}
          </div>
      </div>
    }
}
