use yew::Properties;
use crate::helpers::rotate::Rotate;

#[derive(Properties, PartialEq)]
pub struct SvgProps {
    pub height: String,
    pub width: String,
    pub fill: String,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
}