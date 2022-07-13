use yew::{function_component, html};

use crate::components::rc::rc_progress::common::use_transition_duration;
use crate::components::rc::rc_progress::interface::ProgressProps;

// import * as React from 'react';
// import classNames from 'classnames';
// import { useTransitionDuration, defaultProps } from './common';
// import type { ProgressProps } from './interface';

#[function_component(Line)]
pub fn line(props: &ProgressProps) -> Html {
    let mut default_props = props.default();
    default_props.gap_position = None;
    //   // eslint-disable-next-line no-param-reassign
    //   delete restProps.gapPosition;
    let percent_list = match props.percent.clone() {
        None => Vec::new(),
        Some(percent_value) => match percent_value {
            super::interface::Percent::Number(n) => vec![n],
            super::interface::Percent::NumberVec(vec) => vec,
        },
    };
    //   const percentList = Array.isArray(percent) ? percent : [percent];
    let stroke_color_list = match props.stroke_color.clone() {
        None => Vec::new(),
        Some(stroke_color_value) => match stroke_color_value {
            super::interface::StrokeColorType::BaseStrokeColorType(base_stroke_color_type) => match base_stroke_color_type {
               super::interface::BaseStrokeColorType::String(s) => vec![s],
            },
            super::interface::StrokeColorType::BaseStrokeColorTypeVec(vec) => {
                let mut v = vec![];
                vec.into_iter().for_each(|s| {
                    match s {
                        super::interface::BaseStrokeColorType::String(string) => v.push(string),
                    }
                });
                v
            },
        },
    };
    //   const strokeColorList = Array.isArray(strokeColor) ? strokeColor : [strokeColor];

    let paths = use_transition_duration();
    //   const paths = useTransitionDuration();

    let center = props.stroke_width.unwrap_or(1) / 2;//todo maybe make different default func

    //   const center = strokeWidth / 2;

    let right = 100 - props.stroke_width.unwrap_or(1) / 2;//todo maybe make different default func
    //   const right = 100 - strokeWidth / 2;

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
    //   const pathString = `M ${strokeLinecap === 'round' ? center : 0},${center}
    //          L ${strokeLinecap === 'round' ? right : 100},${center}`;
    let view_box_string = format!("0 0 100 {}", props.stroke_width.unwrap_or(1));//todo maybe make different default func
    //   const viewBoxString = `0 0 100 ${strokeWidth}`;
      let stack_ptg = 0;
    html! {}
    //   return (
    //     <svg
    //       className={classNames(`${prefixCls}-line`, className)}
    //       viewBox={viewBoxString}
    //       preserveAspectRatio="none"
    //       style={style}
    //       {...restProps}
    //     >
    //       <path
    //         className={`${prefixCls}-line-trail`}
    //         d={pathString}
    //         strokeLinecap={strokeLinecap}
    //         stroke={trailColor}
    //         strokeWidth={trailWidth || strokeWidth}
    //         fillOpacity="0"
    //       />
    //       {percentList.map((ptg, index) => {
    //         let dashPercent = 1;
    //         switch (strokeLinecap) {
    //           case 'round':
    //             dashPercent = 1 - strokeWidth / 100;
    //             break;
    //           case 'square':
    //             dashPercent = 1 - strokeWidth / 2 / 100;
    //             break;
    //           default:
    //             dashPercent = 1;
    //             break;
    //         }
    //         const pathStyle = {
    //           strokeDasharray: `${ptg * dashPercent}px, 100px`,
    //           strokeDashoffset: `-${stackPtg}px`,
    //           transition:
    //             transition ||
    //             'stroke-dashoffset 0.3s ease 0s, stroke-dasharray .3s ease 0s, stroke 0.3s linear',
    //         };
    //         const color = strokeColorList[index] || strokeColorList[strokeColorList.length - 1];
    //         stackPtg += ptg;
    //         return (
    //           <path
    //             key={index}
    //             className={`${prefixCls}-line-path`}
    //             d={pathString}
    //             strokeLinecap={strokeLinecap}
    //             stroke={color as string}
    //             strokeWidth={strokeWidth}
    //             fillOpacity="0"
    //             ref={(elem) => {
    //               // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
    //               // React will call the ref callback with the DOM element when the component mounts,
    //               // and call it with `null` when it unmounts.
    //               // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

    //               paths[index] = elem;
    //             }}
    //             style={pathStyle}
    //           />
    //         );
    //       })}
    //     </svg>
    //   );
}
// const Line: React.FC<ProgressProps> = ({
//   className,
//   percent,
//   prefixCls,
//   strokeColor,
//   strokeLinecap,
//   strokeWidth,
//   style,
//   trailColor,
//   trailWidth,
//   transition,
//   ...restProps
// }) => {
//   // eslint-disable-next-line no-param-reassign
//   delete restProps.gapPosition;
//   const percentList = Array.isArray(percent) ? percent : [percent];
//   const strokeColorList = Array.isArray(strokeColor) ? strokeColor : [strokeColor];

//   const paths = useTransitionDuration();

//   const center = strokeWidth / 2;
//   const right = 100 - strokeWidth / 2;
//   const pathString = `M ${strokeLinecap === 'round' ? center : 0},${center}
//          L ${strokeLinecap === 'round' ? right : 100},${center}`;
//   const viewBoxString = `0 0 100 ${strokeWidth}`;
//   let stackPtg = 0;
//   return (
//     <svg
//       className={classNames(`${prefixCls}-line`, className)}
//       viewBox={viewBoxString}
//       preserveAspectRatio="none"
//       style={style}
//       {...restProps}
//     >
//       <path
//         className={`${prefixCls}-line-trail`}
//         d={pathString}
//         strokeLinecap={strokeLinecap}
//         stroke={trailColor}
//         strokeWidth={trailWidth || strokeWidth}
//         fillOpacity="0"
//       />
//       {percentList.map((ptg, index) => {
//         let dashPercent = 1;
//         switch (strokeLinecap) {
//           case 'round':
//             dashPercent = 1 - strokeWidth / 100;
//             break;
//           case 'square':
//             dashPercent = 1 - strokeWidth / 2 / 100;
//             break;
//           default:
//             dashPercent = 1;
//             break;
//         }
//         const pathStyle = {
//           strokeDasharray: `${ptg * dashPercent}px, 100px`,
//           strokeDashoffset: `-${stackPtg}px`,
//           transition:
//             transition ||
//             'stroke-dashoffset 0.3s ease 0s, stroke-dasharray .3s ease 0s, stroke 0.3s linear',
//         };
//         const color = strokeColorList[index] || strokeColorList[strokeColorList.length - 1];
//         stackPtg += ptg;
//         return (
//           <path
//             key={index}
//             className={`${prefixCls}-line-path`}
//             d={pathString}
//             strokeLinecap={strokeLinecap}
//             stroke={color as string}
//             strokeWidth={strokeWidth}
//             fillOpacity="0"
//             ref={(elem) => {
//               // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
//               // React will call the ref callback with the DOM element when the component mounts,
//               // and call it with `null` when it unmounts.
//               // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

//               paths[index] = elem;
//             }}
//             style={pathStyle}
//           />
//         );
//       })}
//     </svg>
//   );
// };

// Line.defaultProps = defaultProps;

// Line.displayName = 'Line';

// export default Line;
