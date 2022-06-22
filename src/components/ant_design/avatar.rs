use crate::components::ant_design::icon::Icon;
use crate::components::ant_design::svg::check_circle::CheckCircle;
use crate::components::ant_design::svg::close::Close;
use crate::components::ant_design::svg::close_circle::CloseCircle;
use crate::components::ant_design::svg::exclamation_circle::ExclamationCircle;
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::theme::Theme;
use crate::components::ant_design::svg::info_circle::InfoCircle;
use colorsys::Hsl;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum AvatarSizeType {
    Large,
    Small,
    Default,
}

#[derive(Debug, PartialEq)]
pub enum AvatarSize {
    Number(u16),
    Type(AvatarSizeType)
}

#[derive(Debug, PartialEq)]
pub enum AvatarShape {
    Circle, 
    Square,
}

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub icon: Html,	
    pub shape: Option<AvatarShape>,
    pub size: Option<AvatarSize>,
    pub src: Option<String>,
    // pub src_set	a list of sources to use for different screen resolutions	string	-	3.11.0 //no examples for it yet in antd docs
    pub alt: Option<String>,
    pub on_error: Option<Callback<()>>
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    html! {
        <span class="ant-avatar ant-avatar-circle">
          <span class="ant-avatar-string" style="transform: scale(1) translateX(-50%);">
          </span>
        </span>
    }
}

