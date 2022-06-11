use yew::{function_component, html, Properties};

#[derive(PartialEq)]
pub enum ButtonType {
    Primary,
    Default,
    Dashed,
    Danger,
}

impl ButtonType {
    pub fn get_class(&self) -> String {//Maybe not a string? another enum?
        match *self {
            ButtonType::Primary => String::from("ant-btn-primary"),
            ButtonType::Default => String::from(""),
            ButtonType::Dashed => String::from("ant-btn-dashed"),
            ButtonType::Danger => String::from("ant-btn-danger"),//None?
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub placeholder: String,
    pub button_type: ButtonType,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = format!("ant-btn {}", props.button_type.get_class());
    html! {
      <button type="button" class={classes}>
        <span>{props.placeholder.clone()}</span>
      </button>
    }
}