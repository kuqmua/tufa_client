use crate::components::ant_design::svg::loading::Loading;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(PartialEq)]
pub enum ButtonType {
    Primary,
    Ghost,
    Dashed,
    Danger,
    Link,
}

#[derive(PartialEq)]
pub enum LoadingProp {
    Bool(bool),
    Delay { delay: u32 },
}

#[derive(PartialEq)]
pub enum Shape {
    Circle,
    Round,
}

#[derive(PartialEq)]
pub enum Size {
    Small,
    Large,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub disabled: Option<()>, //or maybe explicit bool?
    pub ghost: Option<()>,
    pub href: Option<String>,
    pub html_type: Option<String>,
    pub icon: Option<Html>, //Icon Component
    pub loading: Option<LoadingProp>,
    pub shape: Option<Shape>,
    pub size: Option<Size>,
    pub target: Option<String>,
    pub button_type: Option<ButtonType>, //original "type"
    pub on_click: Option<Callback<MouseEvent>>,
    pub block: Option<()>,
    pub placeholder: Option<String>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    //todo: button group
    let block_class = match &props.block {
        None => String::from(""),
        Some(_) => String::from("ant-btn-block"),
    };
    let loading_class = match &props.loading {
        None => String::from(""),
        Some(_) => String::from("ant-btn-loading"),
    };
    let ghost_class = match &props.ghost {
        None => String::from(""),
        Some(_) => String::from("ant-btn-background-ghost"),
    };
    let button_only_class = match &props.placeholder {
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
    let shape_class = match &props.shape {
        None => String::from(""),
        Some(shape) => match shape {
            Shape::Circle => String::from("ant-btn-circle"),
            Shape::Round => String::from("ant-btn-round"), //todo
        },
    };
    let button_type_class = match &props.button_type {
        None => String::from(""),
        Some(button_type) => match button_type {
            ButtonType::Primary => String::from("ant-btn-primary"),
            ButtonType::Ghost => String::from("ant-btn-ghost"),
            ButtonType::Dashed => String::from("ant-btn-dashed"),
            ButtonType::Danger => String::from("ant-btn-danger"),
            ButtonType::Link => String::from("ant-btn-link"),
        },
    };
    let classes = format!(
        "ant-btn {} {} {} {} {} {} {}",
        button_type_class,
        shape_class,
        button_only_class,
        size_class,
        ghost_class,
        loading_class,
        block_class
    );
    let placeholder = match &props.placeholder {
        None => html!(""),
        Some(placeholder) => html! {
          <span>
            {placeholder}
          </span>
        },
    };
    let is_button_disabled = props.disabled.is_some();
    let icon = match props.loading {
        None => match &props.icon {
            None => html! {},
            Some(icon) => html! {
              <i class="anticon">
                {icon.clone()}
              </i>
            },
        },
        Some(_) => html! {
          <i class="anticon">
            <Loading/>
          </i>
        },
    };
    html! {
      <button disabled={is_button_disabled} type="button" class={classes}>
        {icon}
        {placeholder}
      </button>
    }
}
