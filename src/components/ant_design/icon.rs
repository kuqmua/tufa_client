use yew::{function_component, html, Html, Properties};

#[derive(PartialEq)]
pub enum Theme {
    Filled,
    Outlined,
    TwoTone,
}

impl Default for &Theme {
    fn default() -> Self {
        &Theme::Outlined
    }
}

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub inner_html: Html,      //svg icon type
    pub style: Option<String>, //CSSProperties
    // pub theme: Option<Theme>,//explicit svg styles
    // moved into svg
    // pub component: ComponentType<CustomIconComponentProps>//todo
    // pub two_tone_color: //todo
    //pub spin: Option<()> // moved into svg
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    html! {
        <i class="anticon">//todo: aria-label="icon: home" (example)
          {props.inner_html.clone()}
        </i>
    }
}
