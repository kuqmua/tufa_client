use yew::{function_component, html};
use crate::components::ant_design::svg::svg_wrapper_props::SvgWrapperProps;

#[function_component(SvgWrapper)]
pub fn svg_wrapper(props: &SvgWrapperProps) -> Html {
    let spin_class = match &props.spin {
        None => String::from(""),
        Some(_) => String::from("anticon-spin"),
    };
    let rotate_style = match &props.rotate {
        None => String::from(""),
        Some(rotate) => format!("transform: rotate({}deg);", rotate.get_degrees()),
    };
    let view_box = match props.view_box.clone() {
        None => String::from("64 64 896 896"),
        Some(view_box_handle) => view_box_handle,
    };
    html! {
      <svg
        viewBox={view_box}
        focusable="false"
        class={spin_class}
        width={props.width.clone()}
        height={props.height.clone()}
        fill={props.fill.clone()}
        aria-hidden="true"
        style={rotate_style}
      >
        {props.inner_html.clone()}
      </svg>
    }
}