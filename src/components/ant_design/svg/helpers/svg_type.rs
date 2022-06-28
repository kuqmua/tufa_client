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
use svg_component::SvgComponent;

#[derive(Debug, PartialEq, Clone, SvgComponent)]
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