
use std::collections::HashMap;
use crate::components::rc::rc_progress::interface::GapPositionType;


// import * as React from 'react';
// import classNames from 'classnames';
use crate::components::rc::rc_progress::common::use_transition_duration;  
// import { defaultProps, useTransitionDuration } from './common';
use crate::components::rc::rc_progress::interface::ProgressProps;

use super::interface::StrokeLinecapType; 
// import type { ProgressProps } from './interface';
// import useId from './hooks/useId';

pub fn strip_percent_to_number(percent: String) -> String {
  percent.replace("%", "")
}

// function stripPercentToNumber(percent: string) {
//   return +percent.replace('%', '');
// }

// function toArray<T>(value: T | T[]): T[] {
//   const mergedValue = value ?? [];
//   return Array.isArray(mergedValue) ? mergedValue : [mergedValue];
// }

pub const VIEW_BOX_SIZE: i32 = 100;
// const VIEW_BOX_SIZE = 100;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GetCircleStyleStrokeColor {
    String(String),
    Record(HashMap<String, String>),
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
) {
    let step_space = match step_space {
        None => 0.0,//f64::default() ??????
        Some(i) => i,
    };

    let offset_deg = (offset / 100.0) * 360.0 * ((360.0 - gap_degree) / 360.0);
//   const offsetDeg = (offset / 100) * 360 * ((360 - gapDegree) / 360);
    let position_deg = match gap_degree {
        0.0 => 0.0,
        _ => match gap_position {
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
            if (stroke_dash_offset >= perimeter_without_gap) {
                stroke_dash_offset = perimeter_without_gap - 0.01;
              }
        }
    }
}
//   if (strokeLinecap === 'round' && percent !== 100) {
//     strokeDashoffset += strokeWidth / 2;
//     // when percent is small enough (<= 1%), keep smallest value to avoid it's disappearance
//     if (strokeDashoffset >= perimeterWithoutGap) {
//       strokeDashoffset = perimeterWithoutGap - 0.01;
//     }
//   }

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