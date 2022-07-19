use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
// use crate::components::rc::rc_progress::common::use_transition_duration;
use crate::components::rc::rc_progress::interface::ProgressProps;

#[function_component(Line)]
pub fn line(props: &ProgressProps) -> Html {
    let mut default_props = props.default();
    default_props.gap_position = None;
    let percent_list = match props.percent.clone() {
        None => Vec::new(),
        Some(percent_value) => match percent_value {
            super::interface::Percent::Number(n) => vec![n],
            super::interface::Percent::NumberVec(vec) => vec,
        },
    };
    let stroke_color_list = match props.stroke_color.clone() {
        None => Vec::new(),
        Some(stroke_color_value) => match stroke_color_value {
            super::interface::StrokeColorType::BaseStrokeColorType(base_stroke_color_type) => match base_stroke_color_type {
               super::interface::BaseStrokeColorType::String(s) => vec![s],
               super::interface::BaseStrokeColorType::Record(_) => vec![],//todo!
            },
            super::interface::StrokeColorType::BaseStrokeColorTypeVec(vec) => {
                let mut v = vec![];
                vec.into_iter().for_each(|s| {
                    match s {
                        super::interface::BaseStrokeColorType::String(string) => v.push(string),
                        super::interface::BaseStrokeColorType::Record(_) => (),//todo!
                    }
                });
                v
            },
        },
    };
    // let paths = use_transition_duration();
    let center = props.stroke_width.unwrap_or(1.0) / 2.0;//todo maybe make different default func
    // let right = 100.0 - props.stroke_width.unwrap_or(1.0) / 2.0;//todo maybe make different default func
    let first_part = match props.stroke_linecap.clone() {
        None => String::from("0"),//not sure about what
        Some(stroke_linecap_value) => match stroke_linecap_value {
            super::interface::StrokeLinecapType::Round => String::from("center"),
            super::interface::StrokeLinecapType::Butt => String::from("0"),
            super::interface::StrokeLinecapType::Square => String::from("0"),
        },
    };
    let second_part = match props.stroke_linecap.clone() {
        None => String::from("100"),//not sure about what
        Some(stroke_linecap_value) => match stroke_linecap_value {
            super::interface::StrokeLinecapType::Round => String::from("right"),
            super::interface::StrokeLinecapType::Butt => String::from("100"),
            super::interface::StrokeLinecapType::Square => String::from("100"),
        },
    };
    let path_string = format!("M {}{} L {}{}", first_part, center, second_part, center);
    let view_box_string = format!("0 0 100 {}", props.stroke_width.unwrap_or(1.0));//todo maybe make different default func
    let mut stack_ptg = 0.0;
    let gap_position = match props.clone().gap_position {
        None => String::from("bottom"),
        Some(gp) => gp.get_value(),
    };
    let on_click = match props.clone().on_click {
        None => Callback::from(|_: MouseEvent|{}),
        Some(oc) => oc,
    };
    let stroke_width = match (props.clone().trail_width, props.clone().stroke_width) {
        (None, None) => String::from("1"),
        (None, Some(s)) => s.to_string(),
        (Some(t), None) => t.to_string(),
        (Some(t), Some(_s)) => t.to_string(),//coz its first in ts code
    };
    let stroke_linecap = match props.clone().stroke_linecap {
        None => String::from("round"),
        Some(stroke_linecap_type) => stroke_linecap_type.get_value(),
    };
    let percent_list_mapped = percent_list.into_iter().enumerate().map(|(index, ptg)| {
        let dash_percent = match props.clone().stroke_linecap {
                None => {
                    1.0
                },
                Some(stroke_linecap_type) => {
                    let stroke_width = props.clone().stroke_width.unwrap_or(1.0);
                    match stroke_linecap_type {
                        super::interface::StrokeLinecapType::Round => {
                            1.0 - stroke_width / 100.0
                        },
                        super::interface::StrokeLinecapType::Butt => {
                            1.0 - stroke_width / 100.0
                        },
                        super::interface::StrokeLinecapType::Square => {
                            1.0 - stroke_width / 100.0
                        },
                    }
            },
        };
        let stroke_dash_array =  format!("{}px, 100px", ptg as f64 * dash_percent);
        let stroke_dash_offset = format!("-{}px", stack_ptg);
        let transition = match props.transition.clone() {
            None => String::from("stroke-dashoffset 0.3s ease 0s, stroke-dasharray .3s ease 0s, stroke 0.3s linear"),
            Some(t) => t,
        };
        let path_style = format!("
            stroke_dash_array: {};
            stroke_dash_offset: {};
            transition: {};
        ",
          stroke_dash_array, 
          stroke_dash_offset, 
          transition
        );
        let color = match stroke_color_list.get(index) {
            None => match stroke_color_list.is_empty() {
                true => String::from("#2db7f5"),//default
                false => match stroke_color_list.last() {
                    None => String::from("#2db7f5"),//default
                    Some(element) => element.to_string(),
                },
            },
            Some(element) => element.to_string(),
        };
        stack_ptg += ptg;
        html!{
            <path
                key={index}
                class={format!("{}-line-path", props.prefix_cls.clone().unwrap_or_else(|| String::from("rc-progress")))}//default
                d={path_string.clone()}
                stroke_linecap={stroke_linecap.clone()}
                stroke={color.clone()}
                stroke_width={stroke_width.clone()}
                fill_opacity="0"
                // ref={|elem} -> {
                //   // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
                //   // React will call the ref callback with the DOM element when the component mounts,
                //   // and call it with `null` when it unmounts.
                //   // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

                //   paths[index] = elem;
                // }}
                style={path_style.clone()}
            />
        }
    }).collect::<Vec<Html>>();
    html! {
        <svg
          class={format!("{}-line", props.clone().prefix_cls.unwrap_or_else(|| String::from("rc-progress")))}//classNames(`${prefixCls}-line`, className)
          viewBox={view_box_string}
          preserve_aspect_ratio="none"
          style={props.clone().style.unwrap_or_else(|| String::from(""))}
          id={props.clone().id}
        //   gap_degree={props.clone().gap_degree}//todo
          gap_position={gap_position}
          onclick={on_click}
        //   steps={props.clone().steps}//todo
        >
          <path
            class={format!("{}-line-trail", props.clone().prefix_cls.unwrap_or_else(|| String::from("rc-progress")))}
            d={path_string}
            stroke_linecap={stroke_linecap}
            stroke={props.clone().trail_color.unwrap_or_else(|| String::from("#D9D9D9"))}
            stroke_width={stroke_width}
            fill_opacity="0"
          />
          {percent_list_mapped}
        </svg>
    }
}
// Line.displayName = 'Line';
