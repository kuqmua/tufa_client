use crate::components::feed::button_wrapper::ButtonWrapper;
use crate::components::svg::expand_more::ExpandMore;
use yew::{function_component, html, Callback};

#[function_component(ExpandMoreButton)]
pub fn expand_more_button() -> Html {
    let callback = Callback::from(move |_| {});
    let icon_size = "24px".to_owned();
    let html_handle = html! {<ExpandMore height={icon_size.clone()} width={icon_size} fill={"#A2B0B9".to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={callback}/>
    }
}
