use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::theme::Theme;
use crate::helpers::rotate::Rotate;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct SvgProps {
    pub height: Option<String>,
    pub width: Option<String>,
    pub fill: Option<FillWith>,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
    pub theme: Option<Theme>,
}
