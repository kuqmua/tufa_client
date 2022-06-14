use yew::{Properties, Html};
use crate::helpers::rotate::Rotate;

#[derive(Properties, PartialEq)]
pub struct SvgWrapperProps {
    pub height: String,
    pub width: String,
    pub fill: String,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
    pub view_box: Option<String>,
    pub inner_html: Html,
}