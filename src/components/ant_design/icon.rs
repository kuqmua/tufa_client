use yew::{function_component, html, Properties, Html};

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
    pub inner_html: Html,//svg icon type
    pub style: Option<String>,//CSSProperties	
    pub theme: Theme,
    pub spin: Option<()>,
    pub rotate: Option<u16>,//must be between 0-360 degrees
    // pub component: ComponentType<CustomIconComponentProps>//todo
    // pub two_tone_color: //todo
}

#[function_component(Button)]
pub fn icon(props: &IconProps) -> Html {
    html! {
        <i class="anticon">//todo: aria-label="icon: home" (example)
          {props.inner_html.clone()}
        </i>
    }
}