use crate::components::feed::buttons::button_wrapper::ButtonWrapper;
use crate::components::svg::share::Share;
use yew::{function_component, html, Callback};

#[function_component(ShareButton)]
pub fn share_button() -> Html {
    let callback = Callback::from(move |_| {});
    let icon_size = "24px".to_owned();
    let html_handle =
        html! {<Share height={icon_size.clone()} width={icon_size} fill={"#5B6267".to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={callback}/>
    }
}
