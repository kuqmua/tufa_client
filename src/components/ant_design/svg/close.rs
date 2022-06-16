use yew::{function_component, html};
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_wrapper::SvgWrapper;
use crate::constants::WHITE_HSL;

#[function_component(Close)]
pub fn close(props: &SvgProps) -> Html {
    let inner_html = html!{
        <path d="M563.8 512l262.5-312.9c4.4-5.2.7-13.1-6.1-13.1h-79.8c-4.7 0-9.2 2.1-12.3 5.7L511.6 449.8 295.1 191.7c-3-3.6-7.5-5.7-12.3-5.7H203c-6.8 0-10.5 7.9-6.1 13.1L459.4 512 196.9 824.9A7.95 7.95 0 0 0 203 838h79.8c4.7 0 9.2-2.1 12.3-5.7l216.5-258.1 216.5 258.1c3 3.6 7.5 5.7 12.3 5.7h79.8c6.8 0 10.5-7.9 6.1-13.1L563.8 512z">
        </path>
    };
    let fill = match props.fill.clone() {
      None => WHITE_HSL.clone(),
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