use yew::virtual_dom::AttrValue;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub inner_html: Html,         //svg icon type
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
          {props.inner_html.clone()}
        </i>
    }
}
