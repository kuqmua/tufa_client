use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback, Html};

#[derive(PartialEq)]
pub enum ButtonType {
    Primary,
    Ghost,
    Default,
    Dashed,
    Danger,
    Link,
}

impl ButtonType {
    pub fn get_class(&self) -> String {//Maybe not a string? another enum?
        match *self {
            ButtonType::Primary => String::from("ant-btn-primary"),
            ButtonType::Ghost => String::from(""),//todo
            ButtonType::Default => String::from(""),//None?
            ButtonType::Dashed => String::from("ant-btn-dashed"),
            ButtonType::Danger => String::from("ant-btn-danger"),
            ButtonType::Link => String::from(""),//todo
        }
    }
}

impl Default for &ButtonType {
    fn default() -> Self {
        &ButtonType::Default
    }
}

#[derive(PartialEq)]
pub enum Loading {
    Bool(bool),
    Delay{
        delay: u32,
    }
}

#[derive(PartialEq)]
pub enum Shape {
    Circle, 
    Round,
}

#[derive(PartialEq)]
pub enum Size {
    Default, 
    Small, 
    Large,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub disabled: Option<bool>,//maybe i wrong to put Option here
    pub ghost: Option<bool>,
    pub href: Option<String>,
    pub html_type: Option<String>,
    pub icon: Option<String>,//Icon Component
    pub loading: Option<Loading>,
    pub shape: Option<Shape>,
    pub size: Option<Size>,	
    pub target: Option<String>,
    pub button_type: Option<ButtonType>,//original "type"
    pub on_click: Option<Callback<MouseEvent>>,
    pub block: Option<bool>,
    pub inner_html: Option<Html>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = format!("ant-btn {}", props.button_type.as_ref().unwrap_or_default().get_class());
    html! {
      <button type="button" class={classes}>
        <span>{props.inner_html.clone().unwrap_or_else(|| html!(""))}</span>
      </button>
    }
}