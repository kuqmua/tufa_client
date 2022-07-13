use crate::components::rc::rc_progress::interface::ProgressProps;
use crate::components::rc::rc_progress::interface::Percent;
use crate::components::rc::rc_progress::interface::StrokeColorType;
use crate::components::rc::rc_progress::interface::StrokeLinecapType;
use crate::components::rc::rc_progress::interface::BaseStrokeColorType;
use super::interface::GapPositionType;
use lazy_static::lazy_static;
use yew::{function_component, html, use_state, Callback, Html};
use js_sys::Date;
use yew::functional::use_effect;
// import { useRef, useEffect } from 'react';
// import type { ProgressProps } from './interface';

//implemented through trait Default 

// export const defaultProps: Partial<ProgressProps> = {
//   className: '',
//   percent: 0,
//   prefixCls: 'rc-progress',
//   strokeColor: '#2db7f5',
//   strokeLinecap: 'round',
//   strokeWidth: 1,
//   style: {},
//   trailColor: '#D9D9D9',
//   trailWidth: 1,
//   gapPosition: 'bottom',
// };

pub fn use_transition_duration() -> Vec<Html> {
//   const pathsRef = useRef<SVGPathElement[]>([]);
//   const prevTimeStamp = useRef(null);

  use_effect(|| {
    let now = Date::now();
    // const now = Date.now();
    let updated = false;

    // pathsRef.current.forEach((path) => {
    //   if (!path) {
    //     return;
    //   }

    //   updated = true;
    //   const pathStyle = path.style;
    //   pathStyle.transitionDuration = '.3s, .3s, .3s, .06s';

    //   if (prevTimeStamp.current && now - prevTimeStamp.current < 100) {
    //     pathStyle.transitionDuration = '0s, 0s';
    //   }
    // });

    // if (updated) {
    //   prevTimeStamp.current = Date.now();
    // }
    || {}
  });

  vec![html!{}, html!{}]
//   return pathsRef.current;
}

// export const useTransitionDuration = (): SVGPathElement[] => {
//   const pathsRef = useRef<SVGPathElement[]>([]);
//   const prevTimeStamp = useRef(null);

//   useEffect(() => {
//     const now = Date.now();
//     let updated = false;

//     pathsRef.current.forEach((path) => {
//       if (!path) {
//         return;
//       }

//       updated = true;
//       const pathStyle = path.style;
//       pathStyle.transitionDuration = '.3s, .3s, .3s, .06s';

//       if (prevTimeStamp.current && now - prevTimeStamp.current < 100) {
//         pathStyle.transitionDuration = '0s, 0s';
//       }
//     });

//     if (updated) {
//       prevTimeStamp.current = Date.now();
//     }
//   });

//   return pathsRef.current;
// };