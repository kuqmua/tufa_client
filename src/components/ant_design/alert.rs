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
use yew::{function_component, html, Callback, Html, Properties, use_state};

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
    pub after_close: Option<Callback<MouseEvent>>,
    pub banner: Option<()>,
    pub closable: Option<()>,
    pub close_text: Option<String>,  //Html
    pub description: Option<String>, //Html
    pub icon: Option<Html>,
    pub message: Option<String>,
    pub show_icon: Option<()>,
    pub type_handle: Option<AlertType>,
    pub on_close: Option<Callback<()>>,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let closing = use_state(|| false);
    let closed = use_state(|| false);
    let style = use_state(|| String::from(""));
    let closing_handle_close_clone = closing.clone();
    let on_close_clone = props.on_close.clone();
    let handle_close = Callback::<MouseEvent>::from(move |e: MouseEvent| {
        e.prevent_default();
        // const dom = ReactDOM.findDOMNode(this) as HTMLElement;
        // dom.style.height = "${dom.offsetHeight}px";
        // Magic code
        // 重复一次后才能正确设置 height
        // dom.style.height = `${dom.offsetHeight}px`;
        closing_handle_close_clone.set(true);
        // if let Some(function) = on_close_clone {
        //     // function();
        // }
        // match props.on_close.clone() {
        //     None => Callback::<MouseEvent>::from(move |_| {}),
        //     Some(_) => Callback::<MouseEvent>::from(move |_| {}),
        // };
        // (this.props.onClose || noop)(e);
    });
    let closing_animation_end_clone = closing.clone();
    let closed_animation_end_clone = closed.clone();
    let animation_end = || {
        closing_animation_end_clone.set(false);
        closed_animation_end_clone.set(true);
        // (this.props.afterClose || noop)();
      };
    //
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
        "ant-alert {} {} {}",
        type_handle.get_class(),
        description_class,
        show_icon_class
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
                let close =
                    html! {<Close fill={FillWith::Hsl(Hsl::new(0.0, 100.0, 0.0, Some(1.0)))}/>};
                let icon_inner_html = html! {<Icon inner_html={close} additional_class={String::from("anticon-close")} />};
                html! {
                  <button type="button" class="ant-alert-close-icon" tabindex="0">// onclick={on_close}
                    {icon_inner_html}
                  </button>
                }
            }
            Some(text) => html! {
                <button type="button" class="ant-alert-close-icon" tabindex="0">// onclick={on_close}
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
                    let check_circle =
                        html! {<CheckCircle fill={FillWith::CurrentColor} theme={theme}/>};
                    html! {<Icon inner_html={check_circle} additional_class={String::from("anticon-check-circle ant-alert-icon")}/>}
                }
                AlertType::Info => {
                    let info_circle =
                        html! {<InfoCircle fill={FillWith::CurrentColor} theme={theme}/>};
                    html! {<Icon inner_html={info_circle} additional_class={String::from("anticon-info-circle ant-alert-icon")} />}
                }
                AlertType::Warning => {
                    let exclamation_circle =
                        html! {<ExclamationCircle fill={FillWith::CurrentColor} theme={theme}/>};
                    html! {<Icon inner_html={exclamation_circle} additional_class={String::from("anticon-exclamation-circle ant-alert-icon")} />}
                }
                AlertType::Error => {
                    let close_circle =
                        html! {<CloseCircle fill={FillWith::CurrentColor} theme={theme}/>};
                    html! {<Icon inner_html={close_circle} additional_class={String::from("anticon-close-circle ant-alert-icon")} />}
                }
            }
        }
    };
    let on_close_clone = &*style;
    html! {
      <div data-show="true" class={class} style={on_close_clone.clone()}>
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
}
