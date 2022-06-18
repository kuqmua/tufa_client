use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum AlertType {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub after_close: Option<Callback<MouseEvent>>,
    pub banner: Option<bool>,
    pub closable: Option<bool>,
    pub close_text: Option<String>,  //Html
    pub description: Option<String>, //Html
    pub icon: Option<Html>,
    pub message: Option<String>, //Html
    pub show_icon: Option<bool>,
    pub type_handle: Option<AlertType>,
    pub on_close: Option<Callback<MouseEvent>>,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
      <div data-show="true" class="ant-alert ant-alert-info ant-alert-no-icon">
        <span class="ant-alert-message">
        </span>
        <span class="ant-alert-description">
        </span>
      </div>
    }
}
