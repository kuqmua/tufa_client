use colorsys::Hsl;
use yew::{function_component, html};
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_wrapper::SvgWrapper;

#[function_component(Up)]
pub fn up(props: &SvgProps) -> Html {
    let inner_html = html!{
        <path d="M890.5 755.3L537.9 269.2c-12.8-17.6-39-17.6-51.7 0L133.5 755.3A8 8 0 0 0 140 768h75c5.1 0 9.9-2.5 12.9-6.6L512 369.8l284.1 391.6c3 4.1 7.8 6.6 12.9 6.6h75c6.5 0 10.3-7.4 6.5-12.7z">
        </path>
    };
    let fill = match props.fill.clone() {
      None => Hsl::new(0.0, 100.0, 100.0, Some(1.0)),
      Some(fill) => fill,
    };
    html! {
      <SvgWrapper 
        width={props.width.clone()}
        height={props.height.clone()}
        fill={fill}
        spin={props.spin.clone()}
        rotate={props.rotate.clone()}
        inner_html={inner_html}
      />
    }
}