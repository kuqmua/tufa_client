use yew::html;
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::theme::Theme;
use yew::Html;

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
use crate::helpers::rotate::Rotate;

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

impl SvgType {
    pub fn get_html(
        &self,
        height: Option<String>,
        width: Option<String>,
        fill: Option<FillWith>,
        spin: Option<()>,
        rotate: Option<Rotate>,
        theme: Option<Theme>,
    ) -> Html {
        match *self {
            SvgType::CheckCircle => html!{<CheckCircle {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::CloseCircle => html!{<CloseCircle {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Close => html!{<Close {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Cloud => html!{<Cloud {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Copy => html!{<Copy {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Dislike => html!{<Dislike {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Down => html!{<Down {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::ExclamationCircle => html!{<ExclamationCircle {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Github => html!{<Github {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Heart => html!{<Heart {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::InfoCircle => html!{<InfoCircle {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Like => html!{<Like {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Loading => html!{<Loading {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Login => html!{<Login {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Logout => html!{<Logout {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Reddit => html!{<Reddit {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::ShareAlt => html!{<ShareAlt {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Sync => html!{<Sync {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Twitter => html!{<Twitter {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Up => html!{<Up {height} {width} {fill} {spin} {rotate} {theme}/>},
            SvgType::Youtube => html!{<Youtube {height} {width} {fill} {spin} {rotate} {theme}/>},
        }
    }
}