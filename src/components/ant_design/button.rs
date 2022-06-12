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
            ButtonType::Ghost => String::from("ant-btn-ghost"),
            ButtonType::Default => String::from(""),//None?
            ButtonType::Dashed => String::from("ant-btn-dashed"),
            ButtonType::Danger => String::from("ant-btn-danger"),
            ButtonType::Link => String::from("ant-btn-link"),
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

impl Default for &Shape {
    fn default() -> Self {
        &Shape::Round
    }
}

impl Shape {
    pub fn get_class(&self) -> String {//Maybe not a string? another enum?
        match *self {
          Shape::Circle => String::from("ant-btn-circle"),
          Shape::Round => String::from("ant-btn-round"),//todo
        }
    }
}

#[derive(PartialEq)]
pub enum Size {
    Small, 
    Large,
}

#[derive(PartialEq, Clone)]
pub enum InnerHtmlType {
    Text(String), //Icon Component
    InnerHtml(Html), 
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub disabled: Option<()>,//or maybe explicit bool?
    pub ghost: Option<bool>,
    pub href: Option<String>,
    pub html_type: Option<String>,
    pub icon: Option<Html>,//Icon Component
    pub loading: Option<Loading>,
    pub shape: Option<Shape>,
    pub size: Option<Size>,	
    pub target: Option<String>,
    pub button_type: Option<ButtonType>,//original "type"
    pub on_click: Option<Callback<MouseEvent>>,
    pub block: Option<bool>,
    pub inner_html: Option<InnerHtmlType>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let button_only_class = match &props.inner_html {
        None => match &props.icon {
            None => String::from(""),
            Some(_) => String::from("ant-btn-icon-only"),
        },
        Some(_) => String::from(""),
    };
    let size_class = match &props.size {
        None => String::from(""),
        Some(size) => match size {
            Size::Small => String::from("ant-btn-sm"),
            Size::Large => String::from("ant-btn-lg"),
        },   
    };
    let classes = format!(
        "ant-btn {} {} {} {}", 
        props.button_type.as_ref().unwrap_or_default().get_class(),
        props.shape.as_ref().unwrap_or_default().get_class(),
        button_only_class,
        size_class
    );
    let inner_content = match &props.inner_html {
        None => html!(""),
        Some(inner_html_type) => match inner_html_type {
            InnerHtmlType::Text(text) => html!{ <span>{text}</span>},
            InnerHtmlType::InnerHtml(inner_html) => html!{{inner_html.clone()}},//todo
        },
    };
    let inner_icon = match &props.icon {
        None => html!{},
        Some(icon) => html!{{icon.clone()}},
    };
    let is_button_disabled = props.disabled.is_some();
    html! {
      <button disabled={is_button_disabled} type="button" class={classes}>
        {inner_icon}
        {inner_content}
      </button>
    }
}