use yew::{function_component, html, Callback, Html, Properties};
use crate::components::ant_design::svg::helpers::svg_type::SvgType;

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
    pub icon: Option<SvgType>,
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
    let icon_class = match props.icon.clone() {
        None => String::from(""),
        Some(_) => String::from("ant-avatar-icon"),
    };
    let is_icon = props.icon.clone().is_some();
    let style = format!("{}", size_style);
    let class = format!("ant-avatar {} {} {}", shape_class, size_class, icon_class);
    html! {
        <span class={class} style={style}>
          if is_icon {
            <i aria-label="icon: user" class="anticon anticon-user">
              <svg viewBox="64 64 896 896" focusable="false" class="" data-icon="user" width="1em" height="1em" fill="currentColor" aria-hidden="true">
                <path d="M858.5 763.6a374 374 0 0 0-80.6-119.5 375.63 375.63 0 0 0-119.5-80.6c-.4-.2-.8-.3-1.2-.5C719.5 518 760 444.7 760 362c0-137-111-248-248-248S264 225 264 362c0 82.7 40.5 156 102.8 201.1-.4.2-.8.3-1.2.5-44.8 18.9-85 46-119.5 80.6a375.63 375.63 0 0 0-80.6 119.5A371.7 371.7 0 0 0 136 901.8a8 8 0 0 0 8 8.2h60c4.4 0 7.9-3.5 8-7.8 2-77.2 33-149.5 87.8-204.3 56.7-56.7 132-87.9 212.2-87.9s155.5 31.2 212.2 87.9C779 752.7 810 825 812 902.2c.1 4.4 3.6 7.8 8 7.8h60a8 8 0 0 0 8-8.2c-1-47.8-10.9-94.3-29.5-138.2zM512 534c-45.9 0-89.1-17.9-121.6-50.4S340 407.9 340 362c0-45.9 17.9-89.1 50.4-121.6S466.1 190 512 190s89.1 17.9 121.6 50.4S684 316.1 684 362c0 45.9-17.9 89.1-50.4 121.6S557.9 534 512 534z">
                </path>
              </svg>
            </i>
          }
          else {
            <span class="ant-avatar-string" style="transform: scale(1) translateX(-50%);">
            </span>
          }
        </span>
    }
}
