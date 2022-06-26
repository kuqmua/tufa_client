use super::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_type::SvgType;
use yew::virtual_dom::AttrValue;
use yew::{function_component, html, ChildrenWithProps, Html, Properties};
use yew::{html::ChildrenRenderer, virtual_dom::VChild, Component, Context};

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub svg_type: SvgType,
    // #[prop_or_default]
    // pub children: ChildrenWithProps<SvgProps>, //svg icon type
    pub style: Option<AttrValue>, //CSSProperties
    pub additional_class: Option<AttrValue>,
    // pub theme: Option<Theme>,//explicit svg styles
    // moved into svg
    // pub component: ComponentType<CustomIconComponentProps>//todo
    // pub two_tone_color: //todo
    //pub spin: Option<()> // moved into svg
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let additional_class = match props.additional_class.clone() {
        None => AttrValue::Static(""),
        Some(class) => class,
    };
    let class = format!("anticon {}", additional_class);
    html! {
        <i class={class}>//todo: aria-label="icon: home" (example)
          {props.svg_type.get_html()}
        //   {props.inner_html.clone()}
        //   { for props.children.iter() }
        </i>
    }
}
