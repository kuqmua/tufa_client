use web_sys::MouseEvent;
use yew::Callback;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Percent {
    Number(i32),
    NumberVec(Vec<i32>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CountSpace {
    count: i32,
    space: i32,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Steps {
    Number(i32),
    CountSpace(CountSpace),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProgressProps {
    id: Option<String>,
    stroke_width: Option<i32>,
    trail_width: Option<i32>,
    class_name: Option<String>,
    percent: Option<Percent>,
    stroke_color: Option<StrokeColorType>,
    trail_color: Option<String>,
    stroke_linecap: Option<StrokeLinecapType>,
    prefix_cls: Option<String>,
    style: Option<String>,//React.CSSProperties
    gap_degree: Option<i32>,
    gap_position: Option<GapPositionType>,
    transition: Option<String>,
    on_click: Option<Callback<MouseEvent>>,//React.MouseEventHandler
    steps: Option<Steps>,
  }

impl Default for ProgressProps {
    fn default() -> Self {
        ProgressProps{
            id: None,
            stroke_width: Some(1),
            trail_width: Some(1),
            class_name: Some(String::from("")),
            percent: Some(Percent::Number(0)),
            stroke_color: Some(StrokeColorType::BaseStrokeColorType(BaseStrokeColorType::String(String::from("#2db7f5")))),
            trail_color: Some(String::from("#D9D9D9")),
            stroke_linecap: Some(StrokeLinecapType::Round),
            prefix_cls: Some(String::from("rc-progress")),
            style: Some(String::from("")),//React.CSSProperties
            gap_degree: None,
            gap_position: Some(GapPositionType::Bottom),
            transition: None,
            on_click: None,//React.MouseEventHandler
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
    Record((String, String)),
}

//   export type BaseStrokeColorType = string | Record<string, string>;
  
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StrokeColorType {
    BaseStrokeColorType(BaseStrokeColorType),
    BaseStrokeColorTypeVec(Vec<BaseStrokeColorType>),
}

//   export type StrokeColorType = BaseStrokeColorType | BaseStrokeColorType[];
  
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GapPositionType {
    Top,
    Right,
    Bottom,
    Left,
}

//   export type GapPositionType = 'top' | 'right' | 'bottom' | 'left';
  
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StrokeLinecapType {
    Round, 
    Butt, 
    Square,
}

//   export type StrokeLinecapType = 'round' | 'butt' | 'square';