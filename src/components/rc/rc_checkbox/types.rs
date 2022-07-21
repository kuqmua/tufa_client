use yew::Callback;
use yew::Event;
use yew::FocusEvent;
use yew::MouseEvent;

pub struct RcCheckBoxProps {
    pub prefix_cls: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>, //React.CSSProperties
    pub name: Option<String>,
    pub id: Option<String>,
    pub type_handle: Option<String>,
    pub title: Option<String>,
    //   pub default_checked?: number | boolean;
    //   pub checked?: number | boolean;
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
