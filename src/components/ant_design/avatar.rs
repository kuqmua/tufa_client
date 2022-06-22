use yew::{function_component, html, Callback, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum AvatarShape {
    Circle, 
    Square,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AvatarSizeType {
    Large,
    Small,
    Default,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AvatarSize {
    Number(u16),
    Type(AvatarSizeType)
}

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub icon: Option<Html>,	
    pub shape: Option<AvatarShape>,
    pub size: Option<AvatarSize>,
    pub src: Option<String>,
    // pub src_set	a list of sources to use for different screen resolutions	string	-	3.11.0 //no examples for it yet in antd docs
    pub alt: Option<String>,
    pub on_error: Option<Callback<()>>
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let size = match props.size.clone() {
        None => String::from(""),
        Some(size_type) => match size_type {
          AvatarSize::Number(size) => format!("width: {}px; height: {}px; line-height: {}px; font-size: 18px;", size, size, size),
          AvatarSize::Type(_) => todo!(),
        },
    };
    let style = format!("{}", size);
    html! {
        <span class="ant-avatar ant-avatar-circle" style={style}>
          <span class="ant-avatar-string" style="transform: scale(1) translateX(-50%);">
          </span>
        </span>
    }
}

