use crate::constants::INTERFACE_LINES_COLOR;
use yew::{function_component, html, Properties, Html};
use crate::components::feed::expander::touch_line::TouchLine;

#[derive(Properties, PartialEq)]
pub struct ExpanderProps {
  pub inner_html: Html,
}

#[function_component(Expander)]
pub fn expander(props: &ExpanderProps) -> Html {
    let style_handle = format!(
      "
        height: 400px; 
        width: 466px;
        min-width: 466px;
        background-color: #16202A;
        border-top: 1px solid {};
        border-left: 1px solid {};
        border-right: 1px solid {};
        position: fixed;
        border-radius: 30px 30px 0px 0px;
        bottom: 0px;
        display: flex;
        align-items: center;
        flex-direction: column;
        padding: 10px;
      ",
        INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR
    );
    html! {
      <div
        style={style_handle}
      >
          <TouchLine/>
          {props.inner_html.clone()}
      </div>
    }
}
