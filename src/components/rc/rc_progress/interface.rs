use std::{
    collections::HashMap,
    fmt::{self, Display},
};
use web_sys::MouseEvent;
use yew::{Callback, Properties};

#[derive(Debug, PartialEq, Clone)]
pub enum Percent {
    Number(f64),
    NumberVec(Vec<f64>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CountSpace {
    pub count: f64,
    pub space: f64,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Steps {
    Number(f64),
    CountSpace(CountSpace),
}

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct ProgressProps {
    pub id: Option<String>,
    pub stroke_width: Option<f64>,
    pub trail_width: Option<f64>,
    pub class_name: Option<String>,
    pub percent: Option<Percent>,
    pub stroke_color: Option<StrokeColorType>,
    pub trail_color: Option<String>,
    pub stroke_linecap: Option<StrokeLinecapType>,
    pub prefix_cls: Option<String>,
    pub style: Option<String>, //React.CSSProperties
    pub gap_degree: Option<f64>,
    pub gap_position: Option<GapPositionType>,
    pub transition: Option<String>,
    pub on_click: Option<Callback<MouseEvent>>, //React.MouseEventHandler
    pub steps: Option<Steps>,
}

impl Default for ProgressProps {
    fn default() -> Self {
        ProgressProps {
            id: None,
            stroke_width: Some(1.0),
            trail_width: Some(1.0),
            class_name: Some(String::from("")),
            percent: Some(Percent::Number(0.0)),
            stroke_color: Some(StrokeColorType::BaseStrokeColorType(
                BaseStrokeColorType::String(String::from("#2db7f5")),
            )),
            trail_color: Some(String::from("#D9D9D9")),
            stroke_linecap: Some(StrokeLinecapType::Round),
            prefix_cls: Some(String::from("rc-progress")),
            style: Some(String::from("")), //React.CSSProperties
            gap_degree: None,
            gap_position: Some(GapPositionType::Bottom),
            transition: None,
            on_click: None, //React.MouseEventHandler
            steps: None,
        }
    }
}

impl ProgressProps {
    pub fn default(&self) -> Self {
        ProgressProps {
            id: None,
            stroke_width: Some(1.0),
            trail_width: Some(1.0),
            class_name: Some(String::from("")),
            percent: Some(Percent::Number(0.0)),
            stroke_color: Some(StrokeColorType::BaseStrokeColorType(
                BaseStrokeColorType::String(String::from("#2db7f5")),
            )),
            trail_color: Some(String::from("#D9D9D9")),
            stroke_linecap: Some(StrokeLinecapType::Round),
            prefix_cls: Some(String::from("rc-progress")),
            style: Some(String::from("")), //React.CSSProperties
            gap_degree: None,
            gap_position: Some(GapPositionType::Bottom),
            transition: None,
            on_click: None, //React.MouseEventHandler
            steps: None,
        }
    }
}

// export interface ProgressProps {
//     id?: string;
//     strokeWidth?: number;
//     trailWidth?: number;
//     className?: string;
//     percent?: number | number[];
//     strokeColor?: StrokeColorType;
//     trailColor?: string;
//     strokeLinecap?: StrokeLinecapType;
//     prefixCls?: string;
//     style?: React.CSSProperties;
//     gapDegree?: number;
//     gapPosition?: GapPositionType;
//     transition?: string;
//     onClick?: React.MouseEventHandler;
//     steps?: number | { count: number; space: number };
//   }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseStrokeColorType {
    String(String),
    Record(HashMap<String, String>),
}

impl Display for BaseStrokeColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseStrokeColorType::String(s) => write!(f, "{}", s),
            BaseStrokeColorType::Record(_r) => write!(f, ""), //todo
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StrokeColorType {
    BaseStrokeColorType(BaseStrokeColorType),
    BaseStrokeColorTypeVec(Vec<BaseStrokeColorType>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GapPositionType {
    Top,
    Right,
    Bottom,
    Left,
}

impl GapPositionType {
    pub fn get_value(&self) -> String {
        match self {
            GapPositionType::Top => String::from("top"),
            GapPositionType::Right => String::from("right"),
            GapPositionType::Bottom => String::from("bottom"),
            GapPositionType::Left => String::from("left"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StrokeLinecapType {
    Round,
    Butt,
    Square,
}

impl StrokeLinecapType {
    pub fn get_value(&self) -> String {
        match self {
            StrokeLinecapType::Round => String::from("round"),
            StrokeLinecapType::Butt => String::from("butt"),
            StrokeLinecapType::Square => String::from("square"),
        }
    }
}
