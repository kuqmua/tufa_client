use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Debug, PartialEq, Clone)]
pub enum AlertType {
    Success,
    Info,
    Warning,
    Error,
}

impl AlertType {
    pub fn get_class(&self) -> String {
        match self {
            AlertType::Success => String::from("ant-alert-success"),
            AlertType::Info => String::from("ant-alert-info"),
            AlertType::Warning => String::from("ant-alert-warning"),
            AlertType::Error => String::from("ant-alert-error"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub after_close: Option<Callback<MouseEvent>>,
    pub banner: Option<bool>,
    pub closable: Option<bool>,
    pub close_text: Option<String>,  //Html
    pub description: Option<String>, //Html
    pub icon: Option<Html>,
    pub message: Option<String>,
    pub show_icon: Option<bool>,
    pub type_handle: Option<AlertType>,
    pub on_close: Option<Callback<MouseEvent>>,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let message = match props.message.clone() {
        None => String::from(""),
        Some(msg) => msg,
    };
    let type_handle = match props.type_handle.clone() {
        None => AlertType::Info,
        Some(alert_type) => alert_type,
    };
    let class = format!("ant-alert {} ant-alert-no-icon", type_handle.get_class());
    html! {
      <div data-show="true" class={class}>
        <span class="ant-alert-message">{message}
        </span>
        <span class="ant-alert-description">
        </span>
      </div>
    }
}
