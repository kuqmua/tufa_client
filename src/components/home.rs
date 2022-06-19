use super::ant_design::button::LoadingProp;
use super::ant_design::button::Shape;
use super::ant_design::button::Size;
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
// use crate::components::alert::Alert;
use crate::components::ant_design::alert::Alert;
use crate::components::ant_design::alert::AlertType;
use crate::components::ant_design::button::Button;
use crate::components::ant_design::button::ButtonType;
use crate::components::ant_design::icon::Icon;
use crate::components::ant_design::paragraph::Paragraph;
use crate::components::ant_design::svg::down::Down;
use crate::components::ant_design::svg::github::Github;
use crate::components::ant_design::svg::heart::Heart;
use crate::components::ant_design::svg::helpers::theme::Theme;
use crate::components::ant_design::svg::loading::Loading;
use crate::components::ant_design::svg::up::Up;
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
use colorsys::Hsl;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback};

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
    let drawer_style_left_enum_handle = &*drawer_style_left;
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
    let drawer_style_right_enum_handle = &*drawer_style_right;
    let expander_status_cloned_share = expander_status.clone();
    let expander_status_cloned_expand_more = expander_status.clone();
    let expander_status_cloned_close = expander_status;
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
    let loading = html! {<Loading spin={Some(())} />};
    let icon_inner_html = html! {<Icon inner_html={loading}/>};
    // let rotate = Rotate::new(60).unwrap();
    let f = html! {
    <Up
      fill={FillWith::Hsl(Hsl::new(0.0, 100.0, 50.0, Some(1.0)))}
      // spin={Some(())}
      // rotate={Some(rotate)}
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
      <Paragraph/>
      <Alert
        message={String::from("Error")}
        description={String::from("This is an error message about copywriting.")}
        type_handle={AlertType::Error}
        // closable={Some(())}
        show_icon={Some(())}
      />
      <div data-show="true" class="ant-alert ant-alert-error ant-alert-with-description"><i aria-label="icon: close-circle" class="anticon anticon-close-circle ant-alert-icon"><svg viewBox="64 64 896 896" focusable="false" class="" data-icon="close-circle" width="1em" height="1em" fill="currentColor" aria-hidden="true"><path d="M685.4 354.8c0-4.4-3.6-8-8-8l-66 .3L512 465.6l-99.3-118.4-66.1-.3c-4.4 0-8 3.5-8 8 0 1.9.7 3.7 1.9 5.2l130.1 155L340.5 670a8.32 8.32 0 0 0-1.9 5.2c0 4.4 3.6 8 8 8l66.1-.3L512 564.4l99.3 118.4 66 .3c4.4 0 8-3.5 8-8 0-1.9-.7-3.7-1.9-5.2L553.5 515l130.1-155c1.2-1.4 1.8-3.3 1.8-5.2z"></path><path d="M512 65C264.6 65 64 265.6 64 513s200.6 448 448 448 448-200.6 448-448S759.4 65 512 65zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z"></path></svg></i><span class="ant-alert-message">{"Error"}</span><span class="ant-alert-description">{"This is an error message about copywriting."}</span></div>
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
    let expander_style_clone_close_handle = &*expander_style;
    let expander_inner_html = match *expander_status_clone_for_logic {
        ExpanderStatus::Closed => html! {<Alert/>},
        ExpanderStatus::Share => share_inner_html,
        ExpanderStatus::ExpandMore => expand_more_inner_html,
    };
    html! {
      <>
        <Header
          left_drawer_callback={on_open_left}
          right_drawer_callback={on_open_right}
        />
        <Drawer
          callback={on_close_left}
          style_state={drawer_style_left_enum_handle.clone()}
          drawer_position={DrawerPosition::Left}
          inner_html={inner_html_left}
        />
        <Drawer
          callback={on_close_right}
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
