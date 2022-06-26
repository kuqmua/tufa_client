use crate::components::ant_design::svg::check_circle::CheckCircle;
use crate::components::ant_design::svg::close::Close;
use crate::components::ant_design::svg::close_circle::CloseCircle;
use crate::components::ant_design::svg::cloud::Cloud;
use crate::components::ant_design::svg::copy::Copy;
use crate::components::ant_design::svg::dislike::Dislike;
use crate::components::ant_design::svg::down::Down;
use crate::components::ant_design::svg::exclamation_circle::ExclamationCircle;
use crate::components::ant_design::svg::github::Github;
use crate::components::ant_design::svg::heart::Heart;
use crate::components::ant_design::svg::helpers::svg_component_trait::SvgComponent;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::info_circle::InfoCircle;
use crate::components::ant_design::svg::like::Like;
use crate::components::ant_design::svg::loading::Loading;
use crate::components::ant_design::svg::login::Login;
use crate::components::ant_design::svg::logout::Logout;
use crate::components::ant_design::svg::reddit::Reddit;
use crate::components::ant_design::svg::share_alt::ShareAlt;
use crate::components::ant_design::svg::sync::Sync;
use crate::components::ant_design::svg::twitter::Twitter;
use crate::components::ant_design::svg::up::Up;
use crate::components::ant_design::svg::user::User;
use crate::components::ant_design::svg::youtube::Youtube;
use yew::html;
use yew::virtual_dom::AttrValue;
use yew::Html;

#[derive(Debug, PartialEq, Clone)]
pub enum SvgType {
    CheckCircle(SvgProps),
    CloseCircle(SvgProps),
    Close(SvgProps),
    Cloud(SvgProps),
    Copy(SvgProps),
    Dislike(SvgProps),
    Down(SvgProps),
    ExclamationCircle(SvgProps),
    Github(SvgProps),
    Heart(SvgProps),
    InfoCircle(SvgProps),
    Like(SvgProps),
    Loading(SvgProps),
    Login(SvgProps),
    Logout(SvgProps),
    Reddit(SvgProps),
    ShareAlt(SvgProps),
    Sync(SvgProps),
    Twitter(SvgProps),
    Up(SvgProps),
    User(SvgProps),
    Youtube(SvgProps),
}

//todo: make it a proc macro
impl SvgType {
    pub fn get_html(&self) -> Html {
        match self {
            SvgType::CheckCircle(svg_props) => html! {
              <CheckCircle
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
              />
            },
            SvgType::CloseCircle(svg_props) => html! {
              <CloseCircle
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Close(svg_props) => html! {
              <Close
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Cloud(svg_props) => html! {
              <Cloud
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Copy(svg_props) => html! {
              <Copy
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Dislike(svg_props) => html! {
              <Dislike
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Down(svg_props) => html! {
              <Down
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::ExclamationCircle(svg_props) => html! {
              <ExclamationCircle
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Github(svg_props) => html! {
              <Github
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Heart(svg_props) => html! {
              <Heart
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::InfoCircle(svg_props) => html! {
              <InfoCircle
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Like(svg_props) => html! {
              <Like
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Loading(svg_props) => html! {
              <Loading
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Login(svg_props) => html! {
              <Login
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Logout(svg_props) => html! {
              <Logout
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Reddit(svg_props) => html! {
              <Reddit
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::ShareAlt(svg_props) => html! {
              <ShareAlt
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Sync(svg_props) => html! {
              <Sync
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Twitter(svg_props) => html! {
              <Twitter
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Up(svg_props) => html! {
              <Up
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::User(svg_props) => html! {
              <User
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
            SvgType::Youtube(svg_props) => html! {
              <Youtube
                height={svg_props.height.clone()}
                width={svg_props.width.clone()}
                fill={svg_props.fill.clone()}
                spin={svg_props.spin}
                rotate={svg_props.rotate.clone()}
                theme={svg_props.theme.clone()}
             />
            },
        }
    }
    pub fn get_class(&self) -> AttrValue {
        match *self {
            SvgType::CheckCircle(_) => AttrValue::Static("anticon-check-circle"),
            SvgType::CloseCircle(_) => AttrValue::Static("anticon-close-circle"),
            SvgType::Close(_) => AttrValue::Static("anticon-close"),
            SvgType::Cloud(_) => AttrValue::Static("anticon-cloud"),
            SvgType::Copy(_) => AttrValue::Static("anticon-copy"),
            SvgType::Dislike(_) => AttrValue::Static("anticon-dislike"),
            SvgType::Down(_) => AttrValue::Static("anticon-down"),
            SvgType::ExclamationCircle(_) => AttrValue::Static("anticon-exclamation-circle"),
            SvgType::Github(_) => AttrValue::Static("anticon-github"),
            SvgType::Heart(_) => AttrValue::Static("anticon-heart"),
            SvgType::InfoCircle(_) => AttrValue::Static("anticon-info-circle"),
            SvgType::Like(_) => AttrValue::Static("anticon-like"),
            SvgType::Loading(_) => AttrValue::Static("anticon-loading"),
            SvgType::Login(_) => AttrValue::Static("anticon-login"),
            SvgType::Logout(_) => AttrValue::Static("anticon-logout"),
            SvgType::Reddit(_) => AttrValue::Static("anticon-reddit"),
            SvgType::ShareAlt(_) => AttrValue::Static("anticon-share-alt"),
            SvgType::Sync(_) => AttrValue::Static("anticon-sync"),
            SvgType::Twitter(_) => AttrValue::Static("anticon-twitter"),
            SvgType::Up(_) => AttrValue::Static("anticon-up"),
            SvgType::User(_) => AttrValue::Static("anticon-user"),
            SvgType::Youtube(_) => AttrValue::Static("anticon-youtube"),
        }
    }
}
