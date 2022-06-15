use super::ant_design::button::LoadingProp;
use super::ant_design::button::Shape;
use super::ant_design::button::Size;
use crate::components::ant_design::svg::helpers::theme::Theme;
use crate::components::alert::Alert;
use crate::components::ant_design::button::Button;
use crate::components::ant_design::button::ButtonType;
use crate::components::ant_design::icon::Icon;
use crate::components::ant_design::svg::loading::Loading;
use crate::components::ant_design::svg::github::Github;
use crate::components::ant_design::svg::down::Down;
use crate::components::ant_design::svg::heart::Heart;
use crate::components::drawer::component::Drawer;
use crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState;
use crate::components::drawer::position::DrawerPosition;
use crate::components::feed::expander::component::Expander;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState;
use crate::components::feed::expander::share_content::ShareContent;
use crate::components::feed::posts_list::PostsList;
use crate::components::header::component::Header;
use crate::components::material::pure_material_button_contained::PureMaterialButtonContained;
use crate::components::material::pure_material_button_outlined::PureMaterialButtonOutlined;
use crate::components::material::pure_material_button_text::PureMaterialButtonText;
use crate::components::material::pure_material_checkbox::PureMaterialCheckbox;
use crate::components::material::pure_material_progress_circular::PureMaterialProgressCircular;
use crate::components::material::pure_material_progress_linear::PureMaterialProgressLinear;
use crate::components::material::pure_material_radio::PureMaterialRadio;
use crate::components::material::pure_material_slider::PureMaterialSlider;
use crate::components::material::pure_material_switch::PureMaterialSwitch;
use crate::components::material::pure_material_textfield_filled::PureMaterialTextfieldFilled;
use crate::components::material::pure_material_textfield_outlined::PureMaterialTextfieldOutlined;
use crate::components::material::pure_material_textfield_standard::PureMaterialTextfieldStandard;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::constants::HEADER_HEIGHT_PX;
use crate::helpers::rotate::Rotate;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback};
use colorsys::Hsl;

#[derive(Debug, PartialEq)]
pub enum ExpanderStatus {
    Closed,
    Share,
    ExpandMore,
}

impl ExpanderStatus {
    pub fn striing(&self) -> String {
        match *self {
            ExpanderStatus::Closed => String::from("Closed"),
            ExpanderStatus::Share => String::from("share"),
            ExpanderStatus::ExpandMore => String::from("expand more"),
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let padding_summary = HEADER_HEIGHT_PX + HEADER_BORDER_BOTTOM_PX;
    let style_handle = format!(
        "
        padding-top: {}px;
      ",
        padding_summary
    );
    let expander_status = use_state(|| ExpanderStatus::Closed);
    let expander_status_clone_for_logic = expander_status.clone();
    let drawer_style_left = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_left_cloned_on_open = drawer_style_left.clone();
    let expander_status_clone_drawer_on_open_left = expander_status.clone();
    let on_open_left = Callback::from(move |_| {
        expander_status_clone_drawer_on_open_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_left_cloned_first_another = drawer_style_left_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_left_cloned_first_another
                .set(DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_left_cloned_on_close = drawer_style_left.clone();
    let expander_status_clone_drawer_on_close_left = expander_status.clone();
    let on_close_left = Callback::from(move |_| {
        expander_status_clone_drawer_on_close_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_left_cloned_second_another = drawer_style_left_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_left_cloned_second_another.set(DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_left_enum_handle = &*drawer_style_left.clone();
    ////
    let drawer_style_right = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_right_cloned_on_open = drawer_style_right.clone();
    let expander_status_clone_drawer_on_open_right = expander_status.clone();
    let on_open_right = Callback::from(move |_| {
        expander_status_clone_drawer_on_open_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_right_cloned_first_another = drawer_style_right_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_right_cloned_first_another
                .set(DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_right_cloned_on_close = drawer_style_right.clone();
    let expander_status_clone_drawer_on_close_right = expander_status.clone();
    let on_close_right = Callback::from(move |_| {
        expander_status_clone_drawer_on_close_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_right_cloned_second_another = drawer_style_right_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_right_cloned_second_another.set(DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_right_enum_handle = &*drawer_style_right.clone();
    let expander_status_cloned_share = expander_status.clone();
    let expander_status_cloned_expand_more = expander_status.clone();
    let expander_status_cloned_close = expander_status.clone();
    let share_inner_html = html! {<ShareContent/>};
    let expand_more_inner_html = html! {<ExpandMoreContent/>};
    // <PureMaterialButtonOutlined/>
    // <PureMaterialProgressCircular/>
    // <PureMaterialProgressLinear/>
    // <PureMaterialTextfieldFilled/>
    // <PureMaterialTextfieldOutlined/>
    // <PureMaterialRadio/>
    // <PureMaterialButtonContained/>
    // <PureMaterialButtonText/>
    // <PureMaterialSwitch/>
    // <PureMaterialCheckbox/>
    // <PureMaterialTextfieldStandard/>
    // <PureMaterialSlider/>
    let loading = html!{
      <Loading
        height={String::from("1em")}
        width={String::from("1em")}
        fill={Hsl::new(0.0, 100.0, 100.0, Some(1.0))}
        spin={Some(())}
      />};
    let icon_inner_html = html! {<Icon inner_html={loading}/>};
    let rotate = Rotate::new(60).unwrap();
    let f = html!{
      <Heart
        height={String::from("1em")}
        width={String::from("1em")}
        fill={Hsl::new(0.0, 100.0, 50.0, Some(1.0))}
        // spin={Some(())}
        rotate={Some(rotate)}
        theme={Theme::TwoTone}
      />};
    let g = html! {<Icon inner_html={f}/>};
    let inner_html_left = html! {
      <div
        style="
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
          height: 100%;
          width: 100%;
          padding: 10px;
        "
      >
       <Button
        //  placeholder={String::from("Button")}
        //  disabled={Some(())}
         button_type={ButtonType::Primary}
        //  shape={Shape::Circle}
        icon={Some(icon_inner_html.clone())}
        size={Size::Large}
        // ghost={Some(())}
        // block={Some(())}
        // loading={LoadingProp::Bool(true)}
       />
       <Button
       //  placeholder={String::from("Button")}
       //  disabled={Some(())}
        button_type={ButtonType::Primary}
       //  shape={Shape::Circle}
       icon={Some(g.clone())}
       size={Size::Large}
       // ghost={Some(())}
       // block={Some(())}
       // loading={LoadingProp::Bool(true)}
      />
      </div>
    };
    let inner_html_right = html! {};
    let expander_style = use_state(|| ExpanderChangingStyleState::Initial);
    let expander_style_clone_open_expand_more = expander_style.clone();
    let drawer_style_right_expand_more = drawer_style_right.clone();
    let drawer_style_left_expand_more = drawer_style_left.clone();
    let expander_on_open_expand_more = Callback::from(move |_| {
        drawer_style_right_expand_more.set(DrawerChangingStyleState::Initial);
        drawer_style_left_expand_more.set(DrawerChangingStyleState::Initial);
        if let ExpanderStatus::Closed = *expander_status_cloned_expand_more {
            expander_status_cloned_expand_more.set(ExpanderStatus::ExpandMore);
            expander_style_clone_open_expand_more
                .set(ExpanderChangingStyleState::OpenedBeforeTimeout);
            let expander_style_clone_open_another = expander_style_clone_open_expand_more.clone();
            gloo::timers::callback::Timeout::new(50, move || {
                expander_style_clone_open_another
                    .set(ExpanderChangingStyleState::OpenedAfterTimeout);
            })
            .forget();
        }
    });
    let expander_style_clone_open_share = expander_style.clone();
    let drawer_style_right_share = drawer_style_right.clone();
    let drawer_style_left_share = drawer_style_left.clone();
    let expander_on_open_share: Callback<MouseEvent> = Callback::from(move |_| {
        drawer_style_right_share.set(DrawerChangingStyleState::Initial);
        drawer_style_left_share.set(DrawerChangingStyleState::Initial);
        if let ExpanderStatus::Closed = *expander_status_cloned_share {
            expander_status_cloned_share.set(ExpanderStatus::Share);
            expander_style_clone_open_share.set(ExpanderChangingStyleState::OpenedBeforeTimeout);
            let expander_style_clone_open_another = expander_style_clone_open_share.clone();
            gloo::timers::callback::Timeout::new(50, move || {
                expander_style_clone_open_another
                    .set(ExpanderChangingStyleState::OpenedAfterTimeout);
            })
            .forget();
        }
    });
    let expander_style_clone_close = expander_style.clone();
    let drawer_style_right_expander_on_close = drawer_style_right.clone();
    let drawer_style_left_expander_on_close = drawer_style_left.clone();
    let expander_on_close = Callback::from(move |_| {
        drawer_style_right_expander_on_close.set(DrawerChangingStyleState::Initial);
        drawer_style_left_expander_on_close.set(DrawerChangingStyleState::Initial);
        expander_style_clone_close.set(ExpanderChangingStyleState::ClosedBeforeTimeout);
        let expander_style_clone_close_another = expander_style_clone_close.clone();
        let expander_status_cloned_close_another = expander_status_cloned_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            expander_style_clone_close_another.set(ExpanderChangingStyleState::Initial);
            expander_status_cloned_close_another.set(ExpanderStatus::Closed);
        })
        .forget();
    });
    let expander_style_clone_close_handle = &*expander_style.clone().clone();
    let expander_inner_html = match *expander_status_clone_for_logic {
        ExpanderStatus::Closed => html! {<Alert/>},
        ExpanderStatus::Share => share_inner_html,
        ExpanderStatus::ExpandMore => expand_more_inner_html,
    };
    html! {
      <>
        <Header
          left_drawer_callback={on_open_left.clone()}
          right_drawer_callback={on_open_right.clone()}
        />
        <Drawer
          callback={on_close_left.clone()}
          style_state={drawer_style_left_enum_handle.clone()}
          drawer_position={DrawerPosition::Left}
          inner_html={inner_html_left}
        />
        <Drawer
          callback={on_close_right.clone()}
          style_state={drawer_style_right_enum_handle.clone()}
          drawer_position={DrawerPosition::Right}
          inner_html={inner_html_right}
        />
        <div
          style="
            width: 100%; 
            height: 100%;
            display: flex; 
            justify-content: center; 
            flex-direction: column; 
            align-items: center;
          "
        >
          <div
            style={style_handle}
          >
            <PostsList
              share_callback={expander_on_open_share.clone()}//expander_status_to_share
              expand_more_callback={expander_on_open_expand_more.clone()}//expander_status_to_expand_more
            />
            <Expander
              callback={expander_on_close}
              style_state={expander_style_clone_close_handle.clone()}
              inner_html={expander_inner_html}
            />
          </div>
        </div>
      </>
    }
}
