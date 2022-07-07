// import { placements as rcPlacements } from 'rc-tooltip/lib/placements';

#[derive(Debug, PartialEq, Clone)]
pub struct AutoAdjustOverflowHandle {
    pub adjust_x: u8,
    pub adjust_y: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AutoAdjustOverflowEnabled {
    adjust_x: u8,
    adjust_y: u8,
}
impl AutoAdjustOverflowEnabled {
    pub fn new() -> Self {
        AutoAdjustOverflowEnabled {
            adjust_x: 1,
            adjust_y: 1,
        }
    }
}

// const autoAdjustOverflowEnabled = {
//   adjustX: 1,
//   adjustY: 1,
// };

#[derive(Debug, PartialEq, Clone)]
pub struct AutoAdjustOverflowDisabled {
    adjust_x: u8,
    adjust_y: u8,
}
impl AutoAdjustOverflowDisabled {
    pub fn new() -> Self {
        AutoAdjustOverflowDisabled {
            adjust_x: 0,
            adjust_y: 0,
        }
    }
}

// const autoAdjustOverflowDisabled = {
//   adjustX: 0,
//   adjustY: 0,
// };

static TARGET_OFFSET: &'static [i32]= &[0, 0];

// const targetOffset = [0, 0];

#[derive(Debug, PartialEq, Clone)]
pub enum ZeroOrOne {
    Zero,
    One,
}

impl ZeroOrOne {
    pub fn get_value(&self) -> u8 {
        match *self {
            ZeroOrOne::Zero => 0,
            ZeroOrOne::One => 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdjustOverflow {
    adjust_x: Option<ZeroOrOne>,
    adjust_y: Option<ZeroOrOne>,
}

// export interface AdjustOverflow {
//   adjustX?: 0 | 1;
//   adjustY?: 0 | 1;
// }

#[derive(Debug, PartialEq, Clone)]
pub enum AdjustOverflowOrBool {
    AdjustOverflow(AdjustOverflow),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlacementsConfig {
    arrow_width: Option<u32>,
    horizontal_arrow_shift: Option<u32>,
    vertical_arrow_shift: Option<u32>,
    arrow_point_at_center: Option<bool>,
    auto_adjust_overflow: Option<AdjustOverflowOrBool>,
}

// export interface PlacementsConfig {
//   arrowWidth?: number;
//   horizontalArrowShift?: number;
//   verticalArrowShift?: number;
//   arrowPointAtCenter?: boolean;
//   autoAdjustOverflow?: boolean | AdjustOverflow;
// }

pub fn get_overflow_options(auto_adjust_overflow: AdjustOverflowOrBool) -> AutoAdjustOverflowHandle {
  match auto_adjust_overflow {
    AdjustOverflowOrBool::Boolean(bool_value) => match bool_value {
        true => {
            let enabled = AutoAdjustOverflowEnabled::new();
            AutoAdjustOverflowHandle {
                adjust_x: enabled.adjust_x,
                adjust_y: enabled.adjust_y,
            }
        },
        false => {
            let disabled = AutoAdjustOverflowDisabled::new();
            AutoAdjustOverflowHandle {
                adjust_x: disabled.adjust_x,
                adjust_y: disabled.adjust_y,
            }
        },
    },
    AdjustOverflowOrBool::AdjustOverflow(adjust_overflow) => {
        let disabled = AutoAdjustOverflowDisabled::new();
        let x = match adjust_overflow.adjust_x {
            Some(handle) => handle.get_value(),
            None => disabled.adjust_x,
        };
        let y = match adjust_overflow.adjust_y {
            Some(handle) => handle.get_value(),
            None => disabled.adjust_y,
        };
        AutoAdjustOverflowHandle {
            adjust_x: x,
            adjust_y: y,
        }
    },
  }
}

// export function getOverflowOptions(autoAdjustOverflow: boolean | AdjustOverflow) {
//   if (typeof autoAdjustOverflow === 'boolean') {
//     return autoAdjustOverflow ? autoAdjustOverflowEnabled : autoAdjustOverflowDisabled;
//   }
//   return {
//     ...autoAdjustOverflowDisabled,
//     ...autoAdjustOverflow,
//   };
// }

// export default function getPlacements(config: PlacementsConfig = {}) {
//   const {
//     arrowWidth = 5,
//     horizontalArrowShift = 16,
//     verticalArrowShift = 12,
//     autoAdjustOverflow = true,
//   } = config;
//   const placementMap: any = {
//     left: {
//       points: ['cr', 'cl'],
//       offset: [-4, 0],
//     },
//     right: {
//       points: ['cl', 'cr'],
//       offset: [4, 0],
//     },
//     top: {
//       points: ['bc', 'tc'],
//       offset: [0, -4],
//     },
//     bottom: {
//       points: ['tc', 'bc'],
//       offset: [0, 4],
//     },
//     topLeft: {
//       points: ['bl', 'tc'],
//       offset: [-(horizontalArrowShift + arrowWidth), -4],
//     },
//     leftTop: {
//       points: ['tr', 'cl'],
//       offset: [-4, -(verticalArrowShift + arrowWidth)],
//     },
//     topRight: {
//       points: ['br', 'tc'],
//       offset: [horizontalArrowShift + arrowWidth, -4],
//     },
//     rightTop: {
//       points: ['tl', 'cr'],
//       offset: [4, -(verticalArrowShift + arrowWidth)],
//     },
//     bottomRight: {
//       points: ['tr', 'bc'],
//       offset: [horizontalArrowShift + arrowWidth, 4],
//     },
//     rightBottom: {
//       points: ['bl', 'cr'],
//       offset: [4, verticalArrowShift + arrowWidth],
//     },
//     bottomLeft: {
//       points: ['tl', 'bc'],
//       offset: [-(horizontalArrowShift + arrowWidth), 4],
//     },
//     leftBottom: {
//       points: ['br', 'cl'],
//       offset: [-4, verticalArrowShift + arrowWidth],
//     },
//   };
//   Object.keys(placementMap).forEach(key => {
//     placementMap[key] = config.arrowPointAtCenter
//       ? {
//           ...placementMap[key],
//           overflow: getOverflowOptions(autoAdjustOverflow),
//           targetOffset,
//         }
//       : {
//           ...rcPlacements[key],
//           overflow: getOverflowOptions(autoAdjustOverflow),
//         };

//     placementMap[key].ignoreShake = true;
//   });
//   return placementMap;
// }