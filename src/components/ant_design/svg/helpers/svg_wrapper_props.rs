use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::helpers::rotate::Rotate;
use yew::{Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SvgWrapperProps {
    pub height: Option<String>,
    pub width: Option<String>,
    pub fill: FillWith,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
    pub view_box: Option<String>,
    pub inner_html: Html,
}
