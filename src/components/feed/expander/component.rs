use crate::constants::INTERFACE_LINES_COLOR;
use crate::constants::BACKGROUND_COLOR;
use crate::constants::FEED_WIDTH;
use yew::{function_component, html, Properties, Html};
use crate::components::feed::expander::touch_line::TouchLine;

#[derive(Properties, PartialEq)]
pub struct ExpanderProps {
  pub inner_html: Html,
}

#[function_component(Expander)]
pub fn expander(props: &ExpanderProps) -> Html {
    let border_radius = "30px";
    let style_handle = format!(
      "
        height: 400px; 
        width: {}px;
        min-width: {}px;
        background-color: {};
        border-top: 1px solid {};
        border-left: 1px solid {};
        border-right: 1px solid {};
        position: fixed;
        border-radius: {} {} 0px 0px;
        bottom: 0px;
        display: flex;
        align-items: center;
        flex-direction: column;
        padding: 8px;
      ",
      FEED_WIDTH, FEED_WIDTH, BACKGROUND_COLOR, INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR, border_radius, border_radius
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
