use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct SvgProps {
    pub height: String,
    pub width: String,
    pub fill: String,
    pub spin: Option<()>,
}