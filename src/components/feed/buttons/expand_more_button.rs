use crate::components::feed::buttons::button_wrapper::ButtonWrapper;
use crate::components::svg::expand_more::ExpandMore;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct ExpandMoreProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(ExpandMoreButton)]
pub fn expand_more_button(props: &ExpandMoreProps) -> Html {
    let icon_size = "24px".to_owned();
    let html_handle = html! {<ExpandMore height={icon_size.clone()} width={icon_size} fill={"#5B6267".to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={&props.callback}/>
    }
}
