
use std::collections::HashMap;
use yew::{html, function_component};
use yew::Html;
use crate::components::rc::rc_progress::interface::GapPositionType;


// import * as React from 'react';
// import classNames from 'classnames';
use crate::components::rc::rc_progress::common::use_transition_duration;  
// import { defaultProps, useTransitionDuration } from './common';
use crate::components::rc::rc_progress::interface::ProgressProps;
use crate::components::rc::rc_progress::hooks::use_id::use_id;
use crate::components::rc::rc_progress::interface::Percent;

use super::interface::{StrokeLinecapType, StrokeColorType, BaseStrokeColorType}; 
// import type { ProgressProps } from './interface';
// import useId from './hooks/useId';

pub fn strip_percent_to_number(percent: String) -> String {
  percent.replace("%", "")
}

// function stripPercentToNumber(percent: string) {
//   return +percent.replace('%', '');
// }

pub fn percent_to_array(value: Percent) -> Vec<i32> {
    match value {
        Percent::Number(n) => vec![n],
        Percent::NumberVec(vec) => vec,
    }
}

pub fn stroke_color_to_array(value: StrokeColorType) -> Vec<BaseStrokeColorType> {
    match value {
        StrokeColorType::BaseStrokeColorType(n) => vec![n],
        StrokeColorType::BaseStrokeColorTypeVec(vec) => vec,
    }
}

// function toArray<T>(value: T | T[]): T[] {
//   const mergedValue = value ?? [];
//   return Array.isArray(mergedValue) ? mergedValue : [mergedValue];
// }

pub const VIEW_BOX_SIZE: f64 = 100.0;
// const VIEW_BOX_SIZE = 100;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GetCircleStyleStrokeColor {
    String(String),
    Record(HashMap<String, String>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CircleStyle {
    pub stroke: Option<String>,
    pub stroke_dash_array: String,
    pub stroke_dash_offset: f64,
    pub transform: String,
    pub transform_origin: String,
    pub transition: String,
    pub fill_opacity: f64,
}

pub fn get_circle_style (
  perimeter: f64,
  perimeter_without_gap: f64,
  offset: f64,
  percent: f64,
  rotate_deg: f64,
  gap_degree: f64,//uknown
  gap_position: GapPositionType,//Option<GapPositionType>
  stroke_color: GetCircleStyleStrokeColor,
  stroke_linecap: Option<StrokeLinecapType>,
  stroke_width: f64,//uknown
  step_space: Option<f64>,
) -> CircleStyle {
    let step_space = match step_space {
        None => 0.0,//f64::default() ??????
        Some(i) => i,
    };

    let offset_deg = (offset / 100.0) * 360.0 * ((360.0 - gap_degree) / 360.0);
//   const offsetDeg = (offset / 100) * 360 * ((360 - gapDegree) / 360);
    let position_deg = if gap_degree == 0.0 { 0.0 } else {
        match gap_position {
            GapPositionType::Top => 0.0,
            GapPositionType::Right => 180.0,
            GapPositionType::Bottom => 90.0,
            GapPositionType::Left => -90.0,
        }
    };

//   const positionDeg =
//     gapDegree === 0
//       ? 0
//       : {
//           bottom: 0,
//           top: 180,
//           left: 90,
//           right: -90,
//         }[gapPosition];

let mut stroke_dash_offset = ((100.0 - percent) / 100.0) * perimeter_without_gap;
//   let strokeDashoffset = ((100 - percent) / 100) * perimeterWithoutGap;

if let Some(stroke_linecap_type) = stroke_linecap {
    if let StrokeLinecapType::Round = stroke_linecap_type{
        if percent != 100.0 {
            stroke_dash_offset += stroke_width / 2.0;
            if stroke_dash_offset >= perimeter_without_gap {
                stroke_dash_offset = perimeter_without_gap - 0.01;
              }
        }
    }
}

//   // Fix percent accuracy when strokeLinecap is round
//   // https://github.com/ant-design/ant-design/issues/35009
//   if (strokeLinecap === 'round' && percent !== 100) {
//     strokeDashoffset += strokeWidth / 2;
//     // when percent is small enough (<= 1%), keep smallest value to avoid it's disappearance
//     if (strokeDashoffset >= perimeterWithoutGap) {
//       strokeDashoffset = perimeterWithoutGap - 0.01;
//     }
//   }
  let stroke = match stroke_color {
    GetCircleStyleStrokeColor::String(s) => Some(s),
    GetCircleStyleStrokeColor::Record(_) => None,
};

CircleStyle {
    stroke: stroke,
    stroke_dash_array: format!("{}px ${}", perimeter_without_gap, perimeter),
    stroke_dash_offset: stroke_dash_offset + step_space,
    transform: format!("rotate({}deg)", rotate_deg + offset_deg + position_deg),
    transform_origin: String::from("50% 50%"),
    transition: String::from("stroke-dashoffset .3s ease 0s, stroke-dasharray .3s ease 0s, stroke .3s, stroke-width .06s ease .3s, opacity .3s ease 0s"),
    fill_opacity: 0.0
}
//   return {
//     stroke: typeof strokeColor === 'string' ? strokeColor : undefined,
//     strokeDasharray: `${perimeterWithoutGap}px ${perimeter}`,
//     strokeDashoffset: strokeDashoffset + stepSpace,
//     transform: `rotate(${rotateDeg + offsetDeg + positionDeg}deg)`,
//     transformOrigin: '50% 50%',
//     transition:
//       'stroke-dashoffset .3s ease 0s, stroke-dasharray .3s ease 0s, stroke .3s, stroke-width .06s ease .3s, opacity .3s ease 0s',
//     fillOpacity: 0,
//   };
}

// const getCircleStyle = (
//   perimeter: number,
//   perimeterWithoutGap: number,
//   offset: number,
//   percent: number,
//   rotateDeg: number,
//   gapDegree,
//   gapPosition: ProgressProps['gapPosition'] | undefined,
//   strokeColor: string | Record<string, string>,
//   strokeLinecap: ProgressProps['strokeLinecap'],
//   strokeWidth,
//   stepSpace = 0,
// ) => {
//   const offsetDeg = (offset / 100) * 360 * ((360 - gapDegree) / 360);
//   const positionDeg =
//     gapDegree === 0
//       ? 0
//       : {
//           bottom: 0,
//           top: 180,
//           left: 90,
//           right: -90,
//         }[gapPosition];

//   let strokeDashoffset = ((100 - percent) / 100) * perimeterWithoutGap;
//   // Fix percent accuracy when strokeLinecap is round
//   // https://github.com/ant-design/ant-design/issues/35009
//   if (strokeLinecap === 'round' && percent !== 100) {
//     strokeDashoffset += strokeWidth / 2;
//     // when percent is small enough (<= 1%), keep smallest value to avoid it's disappearance
//     if (strokeDashoffset >= perimeterWithoutGap) {
//       strokeDashoffset = perimeterWithoutGap - 0.01;
//     }
//   }

//   return {
//     stroke: typeof strokeColor === 'string' ? strokeColor : undefined,
//     strokeDasharray: `${perimeterWithoutGap}px ${perimeter}`,
//     strokeDashoffset: strokeDashoffset + stepSpace,
//     transform: `rotate(${rotateDeg + offsetDeg + positionDeg}deg)`,
//     transformOrigin: '50% 50%',
//     transition:
//       'stroke-dashoffset .3s ease 0s, stroke-dasharray .3s ease 0s, stroke .3s, stroke-width .06s ease .3s, opacity .3s ease 0s',
//     fillOpacity: 0,
//   };
// };

#[function_component(Circle)]
pub fn circle(props: &ProgressProps) -> Html {
    // ({
    //     id,
    //     prefixCls,
    //     steps,
    //     strokeWidth,
    //     trailWidth,
    //     gapDegree = 0,
    //     gapPosition,
    //     trailColor,
    //     strokeLinecap,
    //     style,
    //     className,
    //     strokeColor,
    //     percent,
    //     ...restProps
    //   }) => {
        let merged_id = use_id(props.id.clone());
//   const mergedId = useId(id);
let gradient_id = format!("{}-gradient", merged_id);
//   const gradientId = `${mergedId}-gradient`;
let radius = VIEW_BOX_SIZE / 2.0 - props.stroke_width.unwrap_or(1.0) / 2.0;
//   const radius = VIEW_BOX_SIZE / 2 - strokeWidth / 2;
let perimeter = 3.14 * 2.0 * radius;
//   const perimeter = Math.PI * 2 * radius;
let gap_degree = match props.gap_degree {
    None => 0,
    Some(g) => g,
};
let rotate_deg = match gap_degree > 0  {
    true => 90 + gap_degree / 2,
    false => -90,
};
//   const rotateDeg = gapDegree > 0 ? 90 + gapDegree / 2 : -90;
let perimeter_without_gap = perimeter * ((360.0 - gap_degree as f64) / 360.0);
//   const perimeterWithoutGap = perimeter * ((360 - gapDegree) / 360);
let (count, space) = match props.steps.clone() {
    None => (0, 0),
    Some(steps_type) => match steps_type {
      super::interface::Steps::Number(n) => (n, 2),
      super::interface::Steps::CountSpace(count_space) => (count_space.count, count_space.space),
    },
};
//   const { count: stepCount, space: stepSpace } =
//     typeof steps === 'object' ? steps : { count: steps, space: 2 };

let circle_style = get_circle_style(
    perimeter,
    perimeter_without_gap,
    0.0,
    100.0,
    rotate_deg as f64,
    gap_degree as f64,
    props.gap_position.clone().unwrap_or(GapPositionType::Bottom),
    GetCircleStyleStrokeColor::String(props.trail_color.clone().unwrap_or(String::from("#D9D9D9"))),
    props.stroke_linecap.clone(),
    props.stroke_width.unwrap_or(1.0),
    None,
);
//   const circleStyle = getCircleStyle(
//     perimeter,
//     perimeterWithoutGap,
//     0,
//     100,
//     rotateDeg,
//     gapDegree,
//     gapPosition,
//     trailColor,
//     strokeLinecap,
//     strokeWidth,
//   );
let percent_list = percent_to_array(props.percent.clone().unwrap_or(Percent::NumberVec(vec![])));
//   const percentList = toArray(percent);
let stroke_color_list = stroke_color_to_array(props.stroke_color.clone().unwrap_or(StrokeColorType::BaseStrokeColorTypeVec(vec![])));
//   const strokeColorList = toArray(strokeColor);
let mut gradient = None;
for color in stroke_color_list.clone() {
match color {
    BaseStrokeColorType::String(_) => (),
    BaseStrokeColorType::Record(hm) => {
        gradient = Some(hm);
        break;
    },
}
}

//   const gradient = strokeColorList.find((color) => color && typeof color === 'object');

let paths = use_transition_duration();

//   const paths = useTransitionDuration();

let get_stoke_list = || {
    let stack_ptg = 0;
    percent_list.clone().iter().enumerate().map(|(index, ptg)|{
        let color = match (stroke_color_list.clone().get(index), stroke_color_list.get(stroke_color_list.len() - 1)) {
            (None, None) => None,
            (None, Some(c)) => Some(c.clone()),
            (Some(c), None) => Some(c.clone()),
            (Some(c), Some(_)) => Some(c.clone()),
        };
                // const color = strokeColorList[index] || strokeColorList[strokeColorList.length - 1];
        let stroke = match color.clone() {
            None => None,
            Some(color_type) => match color_type {
                BaseStrokeColorType::String(_) => None,
                BaseStrokeColorType::Record(_) => Some(format!("url(#{})", gradient_id)),
            },
        };
    //     const stroke = color && typeof color === 'object' ? `url(#${gradientId})` : undefined;
        let circle_style_for_stack = get_circle_style(
          perimeter,
          perimeter_without_gap,
          stack_ptg as f64,
          *ptg as f64,
          rotate_deg as f64,
          gap_degree as f64,
          props.gap_position.clone().unwrap_or(GapPositionType::Bottom),
          GetCircleStyleStrokeColor::String(color.clone().unwrap_or(BaseStrokeColorType::String(String::from("#D9D9D9"))).to_string()),
          props.stroke_linecap.clone(),
          props.stroke_width.clone().unwrap_or(1.0),
          None
        );
    //     const circleStyleForStack = getCircleStyle(
    //       perimeter,
    //       perimeterWithoutGap,
    //       stackPtg,
    //       ptg,
    //       rotateDeg,
    //       gapDegree,
    //       gapPosition,
    //       color,
    //       strokeLinecap,
    //       strokeWidth,
    //     );
    //     stackPtg += ptg;
    //     return (
    //       <circle
    //         key={index}
    //         className={`${prefixCls}-circle-path`}
    //         r={radius}
    //         cx={VIEW_BOX_SIZE / 2}
    //         cy={VIEW_BOX_SIZE / 2}
    //         stroke={stroke}
    //         strokeLinecap={strokeLinecap}
    //         strokeWidth={strokeWidth}
    //         opacity={ptg === 0 ? 0 : 1}
    //         style={circleStyleForStack}
    //         ref={(elem) => {
    //           // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
    //           // React will call the ref callback with the DOM element when the component mounts,
    //           // and call it with `null` when it unmounts.
    //           // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

    //           paths[index] = elem;
    //         }}
    //       />
    //     );
    //   })
    //   .reverse();
        html!{}
    }).collect::<Vec<Html>>();
    
};
//   const getStokeList = () => {
//     let stackPtg = 0;
//     return percentList
//       .map((ptg, index) => {
//         const color = strokeColorList[index] || strokeColorList[strokeColorList.length - 1];
//         const stroke = color && typeof color === 'object' ? `url(#${gradientId})` : undefined;
//         const circleStyleForStack = getCircleStyle(
//           perimeter,
//           perimeterWithoutGap,
//           stackPtg,
//           ptg,
//           rotateDeg,
//           gapDegree,
//           gapPosition,
//           color,
//           strokeLinecap,
//           strokeWidth,
//         );
//         stackPtg += ptg;
//         return (
//           <circle
//             key={index}
//             className={`${prefixCls}-circle-path`}
//             r={radius}
//             cx={VIEW_BOX_SIZE / 2}
//             cy={VIEW_BOX_SIZE / 2}
//             stroke={stroke}
//             strokeLinecap={strokeLinecap}
//             strokeWidth={strokeWidth}
//             opacity={ptg === 0 ? 0 : 1}
//             style={circleStyleForStack}
//             ref={(elem) => {
//               // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
//               // React will call the ref callback with the DOM element when the component mounts,
//               // and call it with `null` when it unmounts.
//               // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

//               paths[index] = elem;
//             }}
//           />
//         );
//       })
//       .reverse();
//   };

//   const getStepStokeList = () => {
//     // only show the first percent when pass steps
//     const current = Math.round(stepCount * (percentList[0] / 100));
//     const stepPtg = 100 / stepCount;

//     let stackPtg = 0;
//     return new Array(stepCount).fill(null).map((_, index) => {
//       const color = index <= current - 1 ? strokeColorList[0] : trailColor;
//       const stroke = color && typeof color === 'object' ? `url(#${gradientId})` : undefined;
//       const circleStyleForStack = getCircleStyle(
//         perimeter,
//         perimeterWithoutGap,
//         stackPtg,
//         stepPtg,
//         rotateDeg,
//         gapDegree,
//         gapPosition,
//         color,
//         'butt',
//         strokeWidth,
//         stepSpace,
//       );
//       stackPtg +=
//         ((perimeterWithoutGap - circleStyleForStack.strokeDashoffset + stepSpace) * 100) /
//         perimeterWithoutGap;

//       return (
//         <circle
//           key={index}
//           className={`${prefixCls}-circle-path`}
//           r={radius}
//           cx={VIEW_BOX_SIZE / 2}
//           cy={VIEW_BOX_SIZE / 2}
//           stroke={stroke}
//           // strokeLinecap={strokeLinecap}
//           strokeWidth={strokeWidth}
//           opacity={1}
//           style={circleStyleForStack}
//           ref={(elem) => {
//             paths[index] = elem;
//           }}
//         />
//       );
//     });
//   };

//   return (
//     <svg
//       className={classNames(`${prefixCls}-circle`, className)}
//       viewBox={`0 0 ${VIEW_BOX_SIZE} ${VIEW_BOX_SIZE}`}
//       style={style}
//       id={id}
//       {...restProps}
//     >
//       {gradient && (
//         <defs>
//           <linearGradient id={gradientId} x1="100%" y1="0%" x2="0%" y2="0%">
//             {Object.keys(gradient)
//               .sort((a, b) => stripPercentToNumber(a) - stripPercentToNumber(b))
//               .map((key, index) => (
//                 <stop key={index} offset={key} stopColor={gradient[key]} />
//               ))}
//           </linearGradient>
//         </defs>
//       )}
//       {!stepCount && (
//         <circle
//           className={`${prefixCls}-circle-trail`}
//           r={radius}
//           cx={VIEW_BOX_SIZE / 2}
//           cy={VIEW_BOX_SIZE / 2}
//           stroke={trailColor}
//           strokeLinecap={strokeLinecap}
//           strokeWidth={trailWidth || strokeWidth}
//           style={circleStyle}
//         />
//       )}
//       {stepCount ? getStepStokeList() : getStokeList()}
//     </svg>
//   );
  html!{}
}

// const Circle: React.FC<ProgressProps> = ({
//   id,
//   prefixCls,
//   steps,
//   strokeWidth,
//   trailWidth,
//   gapDegree = 0,
//   gapPosition,
//   trailColor,
//   strokeLinecap,
//   style,
//   className,
//   strokeColor,
//   percent,
//   ...restProps
// }) => {
//   const mergedId = useId(id);
//   const gradientId = `${mergedId}-gradient`;
//   const radius = VIEW_BOX_SIZE / 2 - strokeWidth / 2;
//   const perimeter = Math.PI * 2 * radius;
//   const rotateDeg = gapDegree > 0 ? 90 + gapDegree / 2 : -90;
//   const perimeterWithoutGap = perimeter * ((360 - gapDegree) / 360);
//   const { count: stepCount, space: stepSpace } =
//     typeof steps === 'object' ? steps : { count: steps, space: 2 };

//   const circleStyle = getCircleStyle(
//     perimeter,
//     perimeterWithoutGap,
//     0,
//     100,
//     rotateDeg,
//     gapDegree,
//     gapPosition,
//     trailColor,
//     strokeLinecap,
//     strokeWidth,
//   );
//   const percentList = toArray(percent);
//   const strokeColorList = toArray(strokeColor);
//   const gradient = strokeColorList.find((color) => color && typeof color === 'object');

//   const paths = useTransitionDuration();

//   const getStokeList = () => {
//     let stackPtg = 0;
//     return percentList
//       .map((ptg, index) => {
//         const color = strokeColorList[index] || strokeColorList[strokeColorList.length - 1];
//         const stroke = color && typeof color === 'object' ? `url(#${gradientId})` : undefined;
//         const circleStyleForStack = getCircleStyle(
//           perimeter,
//           perimeterWithoutGap,
//           stackPtg,
//           ptg,
//           rotateDeg,
//           gapDegree,
//           gapPosition,
//           color,
//           strokeLinecap,
//           strokeWidth,
//         );
//         stackPtg += ptg;
//         return (
//           <circle
//             key={index}
//             className={`${prefixCls}-circle-path`}
//             r={radius}
//             cx={VIEW_BOX_SIZE / 2}
//             cy={VIEW_BOX_SIZE / 2}
//             stroke={stroke}
//             strokeLinecap={strokeLinecap}
//             strokeWidth={strokeWidth}
//             opacity={ptg === 0 ? 0 : 1}
//             style={circleStyleForStack}
//             ref={(elem) => {
//               // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
//               // React will call the ref callback with the DOM element when the component mounts,
//               // and call it with `null` when it unmounts.
//               // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

//               paths[index] = elem;
//             }}
//           />
//         );
//       })
//       .reverse();
//   };

//   const getStepStokeList = () => {
//     // only show the first percent when pass steps
//     const current = Math.round(stepCount * (percentList[0] / 100));
//     const stepPtg = 100 / stepCount;

//     let stackPtg = 0;
//     return new Array(stepCount).fill(null).map((_, index) => {
//       const color = index <= current - 1 ? strokeColorList[0] : trailColor;
//       const stroke = color && typeof color === 'object' ? `url(#${gradientId})` : undefined;
//       const circleStyleForStack = getCircleStyle(
//         perimeter,
//         perimeterWithoutGap,
//         stackPtg,
//         stepPtg,
//         rotateDeg,
//         gapDegree,
//         gapPosition,
//         color,
//         'butt',
//         strokeWidth,
//         stepSpace,
//       );
//       stackPtg +=
//         ((perimeterWithoutGap - circleStyleForStack.strokeDashoffset + stepSpace) * 100) /
//         perimeterWithoutGap;

//       return (
//         <circle
//           key={index}
//           className={`${prefixCls}-circle-path`}
//           r={radius}
//           cx={VIEW_BOX_SIZE / 2}
//           cy={VIEW_BOX_SIZE / 2}
//           stroke={stroke}
//           // strokeLinecap={strokeLinecap}
//           strokeWidth={strokeWidth}
//           opacity={1}
//           style={circleStyleForStack}
//           ref={(elem) => {
//             paths[index] = elem;
//           }}
//         />
//       );
//     });
//   };

//   return (
//     <svg
//       className={classNames(`${prefixCls}-circle`, className)}
//       viewBox={`0 0 ${VIEW_BOX_SIZE} ${VIEW_BOX_SIZE}`}
//       style={style}
//       id={id}
//       {...restProps}
//     >
//       {gradient && (
//         <defs>
//           <linearGradient id={gradientId} x1="100%" y1="0%" x2="0%" y2="0%">
//             {Object.keys(gradient)
//               .sort((a, b) => stripPercentToNumber(a) - stripPercentToNumber(b))
//               .map((key, index) => (
//                 <stop key={index} offset={key} stopColor={gradient[key]} />
//               ))}
//           </linearGradient>
//         </defs>
//       )}
//       {!stepCount && (
//         <circle
//           className={`${prefixCls}-circle-trail`}
//           r={radius}
//           cx={VIEW_BOX_SIZE / 2}
//           cy={VIEW_BOX_SIZE / 2}
//           stroke={trailColor}
//           strokeLinecap={strokeLinecap}
//           strokeWidth={trailWidth || strokeWidth}
//           style={circleStyle}
//         />
//       )}
//       {stepCount ? getStepStokeList() : getStokeList()}
//     </svg>
//   );
// };

// Circle.defaultProps = defaultProps;

// Circle.displayName = 'Circle';

// export default Circle;