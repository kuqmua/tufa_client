use yew::{function_component, html};
use crate::components::ant_design::svg::svg_props::SvgProps;
use crate::components::ant_design::svg::svg_wrapper::SvgWrapper;

#[function_component(Down)]
pub fn down(props: &SvgProps) -> Html {
    let inner_html = html!{
      <path d="M884 256h-75c-5.1 0-9.9 2.5-12.9 6.6L512 654.2 227.9 262.6c-3-4.1-7.8-6.6-12.9-6.6h-75c-6.5 0-10.3 7.4-6.5 12.7l352.6 486.1c12.8 17.6 39 17.6 51.7 0l352.6-486.1c3.9-5.3.1-12.7-6.4-12.7z">
      </path>
    };
    html! {
      <SvgWrapper 
        width={props.width.clone()}
        height={props.height.clone()}
        fill={props.fill.clone()}
        spin={props.spin.clone()}
        rotate={props.rotate.clone()}
        inner_html={inner_html}
      />
    }
}