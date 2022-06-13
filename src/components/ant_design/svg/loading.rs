use yew::{function_component, html};
use crate::components::ant_design::svg::svg_props::SvgProps;

#[function_component(Loading)]
pub fn loading(props: &SvgProps) -> Html {
    let spin_class = match &props.spin {
        None => String::from(""),
        Some(_) => String::from("anticon-spin"),
    };
    html! {
      <svg
        viewBox="0 0 1024 1024"
        focusable="false"
        class={spin_class}
        data-icon="loading"
        width={props.width.clone()}
        height={props.height.clone()}
        fill={props.fill.clone()}
        aria-hidden="true"
      >
        <path d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 0 0-94.3-139.9 437.71 437.71 0 0 0-139.9-94.3C629 83.6 571.4 72 512 72c-19.9 0-36-16.1-36-36s16.1-36 36-36c69.1 0 136.2 13.5 199.3 40.3C772.3 66 827 103 874 150c47 47 83.9 101.8 109.7 162.7 26.7 63.1 40.2 130.2 40.2 199.3.1 19.9-16 36-35.9 36z">
        </path>
      </svg>
    }
}