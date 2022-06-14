use yew::Properties;
use crate::helpers::rotate::Rotate;
use crate::components::ant_design::svg::helpers::theme::Theme;

#[derive(Properties, PartialEq)]
pub struct SvgProps {
    pub height: String,
    pub width: String,
    pub fill: String,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
    pub theme: Option<Theme>,
}