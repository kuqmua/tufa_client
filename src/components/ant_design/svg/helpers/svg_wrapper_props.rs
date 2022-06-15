use yew::{Properties, Html};
use crate::helpers::rotate::Rotate;
use colorsys::Hsl;

#[derive(Properties, PartialEq)]
pub struct SvgWrapperProps {
    pub height: Option<String>,
    pub width: Option<String>,
    pub fill: Hsl,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
    pub view_box: Option<String>,
    pub inner_html: Html,
}