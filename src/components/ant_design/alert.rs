use crate::components::ant_design::icon::Icon;
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::svg_type::SvgType;
use crate::components::ant_design::svg::helpers::theme::Theme;
use colorsys::Hsl;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback, Properties};

#[derive(Debug, PartialEq, Clone)]
pub enum AlertType {
    Success,
    Info,
    Warning,
    Error,
}

impl AlertType {
    pub fn get_class(&self) -> String {
        match self {
            AlertType::Success => String::from("ant-alert-success"),
            AlertType::Info => String::from("ant-alert-info"),
            AlertType::Warning => String::from("ant-alert-warning"),
            AlertType::Error => String::from("ant-alert-error"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    // pub after_close: Option<Callback<()>>,//animation not done yet, todo
    pub banner: Option<()>,
    pub closable: Option<()>,
    pub close_text: Option<String>,  //Html
    pub description: Option<String>, //Html
    pub icon: Option<SvgType>,
    pub message: Option<String>,
    pub show_icon: Option<()>,
    pub type_handle: Option<AlertType>,
    pub on_close: Option<Callback<()>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AlertChangingStyleState {
    Opened,
    Closing,
    Closed,
}

impl AlertChangingStyleState {
    pub fn get_class(&self) -> String {
        match *self {
            AlertChangingStyleState::Opened => String::from("ant-alert-closable"),
            AlertChangingStyleState::Closing => String::from("ant-alert-closing ant-alert-no-icon ant-alert-closable ant-alert-slide-up-leave ant-alert-slide-up-leave-active"),
            AlertChangingStyleState::Closed => String::from(""),
        }
    }
    pub fn get_value(&self) -> AlertChangingStyle {
        match *self {
            AlertChangingStyleState::Opened => AlertChangingStyle {
                should_render: true,
                height: String::from("100%"),
                opacity: String::from(""),
            },
            AlertChangingStyleState::Closing => AlertChangingStyle {
                should_render: true,
                height: String::from("0%"),
                opacity: String::from(""),
            },
            AlertChangingStyleState::Closed => AlertChangingStyle {
                should_render: false,
                height: String::from("0px"),
                opacity: String::from("0.5"),
            },
        }
    }
    pub fn get_style(&self) -> String {
        let value = self.get_value();
        format!("transition: transform 0.3s; {}", value.height)
    }
}

#[derive(Debug, PartialEq)]
pub struct AlertChangingStyle {
    pub should_render: bool,
    pub height: String,
    pub opacity: String,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    // let style = use_state(|| String::from(""));
    // let cloned_style = style.clone();
    let alert_changing_style = use_state(|| AlertChangingStyleState::Opened);
    let alert_changing_style_second_clone = alert_changing_style.clone();
    let handle_close = {
        let on_close_clone = props.on_close.clone();
        // let after_close_clone = props.after_close.clone();
        // let style_clone = style.clone();
        Callback::<MouseEvent>::from(move |e: MouseEvent| {
            let alert_changing_style_cloned = alert_changing_style.clone();
            e.prevent_default();
            // // const dom = ReactDOM.findDOMNode(this) as HTMLElement;
            // // dom.style.height = "${dom.offsetHeight}px";
            // style_clone.set(String::from("${dom.offsetHeight}px"));
            // // Magic code
            // // 重复一次后才能正确设置 height
            // // dom.style.height = `${dom.offsetHeight}px`;
            // style_clone.set(String::from("${dom.offsetHeight}px"));
            if let Some(on_close) = on_close_clone.clone() {
                on_close.emit(());
            };
            alert_changing_style_cloned.set(AlertChangingStyleState::Closed);
            //AlertChangingStyleState::Closing
            // let after_close_clone_clone = after_close_clone.clone();
            // gloo::timers::callback::Timeout::new(300, move || {
            //     //0.3second from antd.css
            //     // let after_close_clone_clone_clone = after_close_clone_clone.clone();
            //     // if let Some(after_close) = after_close_clone_clone_clone {
            //     //     after_close.emit(());
            //     // };
            //     let alert_changing_style_cloned_cloned = alert_changing_style_cloned.clone();
            //     alert_changing_style_cloned_cloned.set(AlertChangingStyleState::Closed);
            // })
            // .forget();
        })
    };
    let message = match props.message.clone() {
        None => String::from(""),
        Some(msg) => msg,
    };
    let description = match props.description.clone() {
        None => String::from(""),
        Some(desc) => desc,
    };
    let type_handle = match props.type_handle.clone() {
        None => AlertType::Info,
        Some(alert_type) => alert_type,
    };
    let description_class = match props.description.clone() {
        None => String::from(""),
        Some(_) => String::from("ant-alert-with-description"),
    };
    let show_icon_class = match props.show_icon {
        None => String::from("ant-alert-no-icon"),
        Some(_) => String::from(""),
    };
    let class = format!(
        "ant-alert {} {} {}", // {}
        type_handle.get_class(),
        description_class,
        show_icon_class,
        // alert_changing_style_second_clone.get_class(),
    );
    let close_button = match props.closable {
        None => match props.close_text.clone() {
            None => html! {},
            Some(text) => html! {
                <button onclick={handle_close} type="button" class="ant-alert-close-icon" tabindex="0">
                  <span class="ant-alert-close-text">{text}</span></button>
            },
        },
        Some(_) => match props.close_text.clone() {
            None => {
                let svg_type = SvgType::Close;
                let close = html! {{svg_type.get_html(None, None, Some(FillWith::Hsl(Hsl::new(0.0, 100.0, 0.0, Some(1.0)))), None, None, None)}}; //<Close fill={FillWith::Hsl(Hsl::new(0.0, 100.0, 0.0, Some(1.0)))}/>
                let icon_inner_html = html! {<Icon inner_html={close} additional_class={String::from("anticon-close")} />};
                html! {
                  <button onclick={handle_close} type="button" class="ant-alert-close-icon" tabindex="0">// onclick={on_close}
                    {icon_inner_html}
                  </button>
                }
            }
            Some(text) => html! {
                <button onclick={handle_close} type="button" class="ant-alert-close-icon" tabindex="0">// onclick={on_close}
                  <span class="ant-alert-close-text">{text}</span></button>
            },
        },
    };
    let icon = match props.show_icon {
        None => html! {},
        Some(_) => {
            let theme = match props.description.clone() {
                Some(_) => Theme::Outlined,
                None => Theme::Filled,
            };
            match type_handle {
                AlertType::Success => {
                    let svg_type = SvgType::CheckCircle;
                    let check_circle = html! {{svg_type.get_html(None, None, Some(FillWith::CurrentColor), None, None, Some(theme))}};
                    html! {<Icon inner_html={check_circle} additional_class={String::from("anticon-check-circle ant-alert-icon")}/>}
                }
                AlertType::Info => {
                    let svg_type = SvgType::InfoCircle;
                    let info_circle = html! {{svg_type.get_html(None, None, Some(FillWith::CurrentColor), None, None, Some(theme))}};
                    html! {<Icon inner_html={info_circle} additional_class={String::from("anticon-info-circle ant-alert-icon")} />}
                }
                AlertType::Warning => {
                    let svg_type = SvgType::ExclamationCircle;
                    let exclamation_circle = html! {{svg_type.get_html(None, None, Some(FillWith::CurrentColor), None, None, Some(theme))}};
                    html! {<Icon inner_html={exclamation_circle} additional_class={String::from("anticon-exclamation-circle ant-alert-icon")} />}
                }
                AlertType::Error => {
                    let svg_type = SvgType::CloseCircle;
                    let close_circle = html! {{svg_type.get_html(None, None, Some(FillWith::CurrentColor), None, None, Some(theme))}};
                    html! {<Icon inner_html={close_circle} additional_class={String::from("anticon-close-circle ant-alert-icon")} />}
                }
            }
        }
    };
    let should_render = alert_changing_style_second_clone.get_value().should_render;
    // let style_handle = &*cloned_style;
    html! {
      <>
        if should_render {
          <div data-show="true" class={class}>// style={style_handle.clone()}
            {icon}
            <span class="ant-alert-message">
              {message}
            </span>
            <span class="ant-alert-description">
              {description}
            </span>
              {close_button}
          </div>
        }
      </>
    }
}
