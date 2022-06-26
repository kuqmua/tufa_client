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
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::theme::Theme;
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
use crate::helpers::rotate::Rotate;
use yew::html;
use yew::virtual_dom::AttrValue;
use yew::virtual_dom::VChild;
use yew::Html;

#[derive(Debug, PartialEq, Clone)]
pub enum SvgType {
    CheckCircle(VChild<CheckCircle>),
    CloseCircle(VChild<CloseCircle>),
    Close(VChild<Close>),
    Cloud(VChild<Cloud>),
    Copy(VChild<Copy>),
    Dislike(VChild<Dislike>),
    Down(VChild<Down>),
    ExclamationCircle(VChild<ExclamationCircle>),
    Github(VChild<Github>),
    Heart(VChild<Heart>),
    InfoCircle(VChild<InfoCircle>),
    Like(VChild<Like>),
    Loading(VChild<Loading>),
    Login(VChild<Login>),
    Logout(VChild<Logout>),
    Reddit(VChild<Reddit>),
    ShareAlt(VChild<ShareAlt>),
    Sync(VChild<Sync>),
    Twitter(VChild<Twitter>),
    Up(VChild<Up>),
    User(VChild<User>),
    Youtube(VChild<Youtube>),
}

impl SvgType {
    pub fn get_html(
        &self,
        height: Option<AttrValue>,
        width: Option<AttrValue>,
        fill: Option<FillWith>,
        spin: Option<()>,
        rotate: Option<Rotate>,
        theme: Option<Theme>,
    ) -> Html {
        match *self {
            SvgType::CheckCircle(_) => {
                html! {<CheckCircle {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::CloseCircle(_) => {
                html! {<CloseCircle {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::Close(_) => html! {<Close {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Cloud(_) => html! {<Cloud {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Copy(_) => html! {<Copy {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Dislike(_) => {
                html! {<Dislike {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::Down(_) => html! {<Down {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::ExclamationCircle(_) => {
                html! {<ExclamationCircle {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::Github(_) => html! {<Github {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Heart(_) => html! {<Heart {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::InfoCircle(_) => {
                html! {<InfoCircle {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::Like(_) => html! {<Like {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Loading(_) => {
                html! {<Loading {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::Login(_) => html! {<Login {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Logout(_) => html! {<Logout {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Reddit(_) => html! {<Reddit {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::ShareAlt(_) => {
                html! {<ShareAlt {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::Sync(_) => html! {<Sync {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Twitter(_) => {
                html! {<Twitter {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
            SvgType::Up(_) => html! {<Up {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::User(_) => html! {<User {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Youtube(_) => {
                html! {<Youtube {height} {width} {fill} {spin} {rotate} {theme}/>}
            }
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

#[allow(clippy::from_over_into)]
impl Into<Html> for SvgType {
    fn into(self) -> Html {
        match self {
            SvgType::CheckCircle(child) => child.into(),
            SvgType::CloseCircle(child) => child.into(),
            SvgType::Close(child) => child.into(),
            SvgType::Cloud(child) => child.into(),
            SvgType::Copy(child) => child.into(),
            SvgType::Dislike(child) => child.into(),
            SvgType::Down(child) => child.into(),
            SvgType::ExclamationCircle(child) => child.into(),
            SvgType::Github(child) => child.into(),
            SvgType::Heart(child) => child.into(),
            SvgType::InfoCircle(child) => child.into(),
            SvgType::Like(child) => child.into(),
            SvgType::Loading(child) => child.into(),
            SvgType::Login(child) => child.into(),
            SvgType::Logout(child) => child.into(),
            SvgType::Reddit(child) => child.into(),
            SvgType::ShareAlt(child) => child.into(),
            SvgType::Sync(child) => child.into(),
            SvgType::Twitter(child) => child.into(),
            SvgType::Up(child) => child.into(),
            SvgType::User(child) => child.into(),
            SvgType::Youtube(child) => child.into(),
        }
    }
}
