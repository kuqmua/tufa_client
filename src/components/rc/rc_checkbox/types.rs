use yew::Callback;
use yew::Event;
use yew::FocusEvent;
use yew::MouseEvent;
use yew::Properties;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct RcCheckBoxProps {
    pub prefix_cls: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>, //React.CSSProperties
    pub name: Option<String>,
    pub id: Option<String>,
    pub type_handle: Option<InputType>,
    pub title: Option<String>,
    pub default_checked: Option<()>, //?: number | boolean;
    pub checked: Option<()>,         //?: number | boolean;
    pub disabled: Option<bool>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_change: Option<Callback<Event>>,
    pub on_click: Option<Callback<MouseEvent>>,
    //   pub tab_index?: string | number;
    pub read_only: Option<bool>,
    pub required: Option<bool>,
    pub auto_focus: Option<bool>,
    //   pub value?: any;
}
