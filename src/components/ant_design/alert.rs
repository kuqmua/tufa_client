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
    pub after_close: Callback<MouseEvent>,
    pub banner: bool,
    pub closable: bool,
    pub close_text: String,  //Html
    pub description: String, //Html
    pub icon: Html,
    pub message: String, //Html
    pub show_icon: bool,
    pub type_handle: AlertType,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {}
}
