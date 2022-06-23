use crate::components::ant_design::svg::check_circle::CheckCircle;
use crate::components::ant_design::svg::close_circle::CloseCircle;
use crate::components::ant_design::svg::close::Close;
use crate::components::ant_design::svg::cloud::Cloud;
use crate::components::ant_design::svg::copy::Copy;
use crate::components::ant_design::svg::dislike::Dislike;
use crate::components::ant_design::svg::down::Down;
use crate::components::ant_design::svg::exclamation_circle::ExclamationCircle;
use crate::components::ant_design::svg::github::Github;
use crate::components::ant_design::svg::heart::Heart;
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
use crate::components::ant_design::svg::youtube::Youtube;

#[derive(Debug, PartialEq, Clone)]
pub enum SvgType {
    CheckCircle,
    CloseCircle,
    Close,
    Cloud,
    Copy,
    Dislike,
    Down,
    ExclamationCircle,
    Github,
    Heart,
    InfoCircle,
    Like,
    Loading,
    Login,
    Logout,
    Reddit,
    ShareAlt,
    Sync,
    Twitter,
    Up,
    Youtube
}