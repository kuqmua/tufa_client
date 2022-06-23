use yew::{function_component, html, Callback, Html, Properties};

#[derive(Debug, PartialEq, Clone)]
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
    Type(AvatarSizeType),
}

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub icon: Option<Html>,
    pub shape: Option<AvatarShape>,
    pub size: Option<AvatarSize>,
    pub src: Option<String>,
    // pub src_set	a list of sources to use for different screen resolutions	string	-	3.11.0 //no examples for it yet in antd docs
    pub alt: Option<String>,
    pub on_error: Option<Callback<()>>,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let size_style: String;
    let size_class: String;
    match props.size.clone() {
        None => {
            size_style = String::from("");
            size_class = String::from("");
        }
        Some(size_type) => match size_type {
            AvatarSize::Number(size) => {
                size_style = format!(
                    "width: {}px; height: {}px; line-height: {}px; font-size: 18px;",
                    size, size, size
                );
                size_class = String::from("");
            }
            AvatarSize::Type(size_type) => match size_type {
                AvatarSizeType::Large => {
                    size_style = String::from("");
                    size_class = String::from("ant-avatar-lg");
                }
                AvatarSizeType::Small => {
                    size_style = String::from("");
                    size_class = String::from("ant-avatar-sm");
                }
                AvatarSizeType::Default => {
                    size_style = String::from("");
                    size_class = String::from("");
                }
            },
        },
    };
    let shape_class = match props.shape.clone() {
        None => String::from("ant-avatar-circle"),
        Some(shape_type) => match shape_type {
          AvatarShape::Circle => String::from("ant-avatar-circle"),
          AvatarShape::Square => String::from("ant-avatar-square"),
        },
    };
    let style = format!("{}", size_style);
    let class = format!("ant-avatar {} {}", shape_class, size_class);
    html! {
        <span class={class} style={style}>
          <span class="ant-avatar-string" style="transform: scale(1) translateX(-50%);">
          </span>
        </span>
    }
}
