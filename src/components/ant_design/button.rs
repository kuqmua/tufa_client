use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback, Html};
use crate::components::ant_design::svg::loading::Loading;

#[derive(PartialEq)]
pub enum ButtonType {
    Primary,
    Ghost,
    Dashed,
    Danger,
    Link,
}

// impl ButtonType {
//     pub fn get_class(&self) -> String {//Maybe not a string? another enum?
//         match *self {
//             ButtonType::Primary => String::from("ant-btn-primary"),
//             ButtonType::Ghost => String::from("ant-btn-ghost"),
//             ButtonType::Dashed => String::from("ant-btn-dashed"),
//             ButtonType::Danger => String::from("ant-btn-danger"),
//             ButtonType::Link => String::from("ant-btn-link"),
//         }
//     }
// }

#[derive(PartialEq)]
pub enum LoadingProp {
    Bool(bool),
    Delay{
        delay: u32,
    }
}

#[derive(PartialEq)]
pub enum Shape {
    Circle, 
    Round,
}

// impl Default for &Shape {
//     fn default() -> Self {
//         &Shape::Round
//     }
// }

// impl Shape {
//     pub fn get_class(&self) -> String {//Maybe not a string? another enum?
//         match *self {
//           Shape::Circle => String::from("ant-btn-circle"),
//           Shape::Round => String::from("ant-btn-round"),//todo
//         }
//     }
// }

#[derive(PartialEq)]
pub enum Size {
    Small, 
    Large,
}

#[derive(PartialEq, Clone)]
pub enum InnerHtmlType {
    Text(String), //Icon Component
    InnerHtml(Html), 
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub disabled: Option<()>,//or maybe explicit bool?
    pub ghost: Option<()>,
    pub href: Option<String>,
    pub html_type: Option<String>,
    pub icon: Option<Html>,//Icon Component
    pub loading: Option<LoadingProp>,
    pub shape: Option<Shape>,
    pub size: Option<Size>,	
    pub target: Option<String>,
    pub button_type: Option<ButtonType>,//original "type"
    pub on_click: Option<Callback<MouseEvent>>,
    pub block: Option<()>,
    pub inner_html: Option<InnerHtmlType>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    //todo: button group
    let block_class = match &props.block {
        None => String::from(""),
        Some(_) => String::from("ant-btn-block"),
    };
    let loading_class = match &props.loading {
        None => String::from(""),
        Some(_) => String::from("ant-btn-loading"),
    };
    let ghost_class = match &props.ghost {
        None => String::from(""),
        Some(_) => String::from("ant-btn-background-ghost"),
    };
    let button_only_class = match &props.inner_html {
        None => match &props.icon {
            None => String::from(""),
            Some(_) => String::from("ant-btn-icon-only"),
        },
        Some(_) => String::from(""),
    };
    let size_class = match &props.size {
        None => String::from(""),
        Some(size) => match size {
            Size::Small => String::from("ant-btn-sm"),
            Size::Large => String::from("ant-btn-lg"),
        },   
    };
    let shape_class = match &props.shape {
        None => String::from(""),
        Some(shape) => match shape {
          Shape::Circle => String::from("ant-btn-circle"),
          Shape::Round => String::from("ant-btn-round"),//todo
        },   
    };
    let button_type_class = match &props.button_type {
        None => String::from(""),
        Some(button_type) => match button_type {
            ButtonType::Primary => String::from("ant-btn-primary"),
            ButtonType::Ghost => String::from("ant-btn-ghost"),
            ButtonType::Dashed => String::from("ant-btn-dashed"),
            ButtonType::Danger => String::from("ant-btn-danger"),
            ButtonType::Link => String::from("ant-btn-link"),
        },   
    };
    let classes = format!(
        "ant-btn {} {} {} {} {} {} {}", 
        button_type_class,
        shape_class,
        button_only_class,
        size_class,
        ghost_class,
        loading_class,
        block_class
    );
    let inner_content = match &props.inner_html {
        None => html!(""),
        Some(inner_html_type) => match inner_html_type {
            InnerHtmlType::Text(text) => 
            // {
            //     match &props.size {
            //       None => {
            //         if &props.loading.is_some() || &props.icon.is_some() {
            //             return 
            //         }
            //       },
            //       Some(_) => todo!(),
            //     }

            // }
              html!{ 
                <span
                  style="
                    margin-left: 8px;
                  "  
                >
                  {text}
                </span>
              }
              ,
            InnerHtmlType::InnerHtml(inner_html) => 
              html!{
                <div
                  style="
                    margin-left: 8px;
                  "
                >
                  {inner_html.clone()}
                </div>
              },//todo
        },
    };
    let inner_icon = match &props.icon {
        None => html!{},
        Some(icon) => html!{{icon.clone()}},
    };
    let is_button_disabled = props.disabled.is_some();
    let icon = match props.loading {
        None => inner_icon,
        Some(_) => html!{<Loading/>},//loading icon html
    };
    html! {
    <>
    <button disabled={is_button_disabled} type="button" class={classes}>
    {icon}
    {inner_content}
  </button>

      <button 
        type="button" 
        class="ant-btn ant-btn-primary ant-btn-sm ant-btn-loading"
      >
        <i aria-label="icon: loading" class="anticon anticon-loading">
          <svg viewBox="0 0 1024 1024" focusable="false" class="anticon-spin" data-icon="loading" width="1em" height="1em" fill="currentColor" aria-hidden="true">
            <path d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 0 0-94.3-139.9 437.71 437.71 0 0 0-139.9-94.3C629 83.6 571.4 72 512 72c-19.9 0-36-16.1-36-36s16.1-36 36-36c69.1 0 136.2 13.5 199.3 40.3C772.3 66 827 103 874 150c47 47 83.9 101.8 109.7 162.7 26.7 63.1 40.2 130.2 40.2 199.3.1 19.9-16 36-35.9 36z">
            </path>
          </svg>
        </i>
        <span>{"Loading"}</span>
      </button>
      <button type="button" class="ant-btn ant-btn-primary ant-btn-loading"><i aria-label="icon: loading" class="anticon anticon-loading"><svg viewBox="0 0 1024 1024" focusable="false" class="anticon-spin" data-icon="loading" width="1em" height="1em" fill="currentColor" aria-hidden="true"><path d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 0 0-94.3-139.9 437.71 437.71 0 0 0-139.9-94.3C629 83.6 571.4 72 512 72c-19.9 0-36-16.1-36-36s16.1-36 36-36c69.1 0 136.2 13.5 199.3 40.3C772.3 66 827 103 874 150c47 47 83.9 101.8 109.7 162.7 26.7 63.1 40.2 130.2 40.2 199.3.1 19.9-16 36-35.9 36z"></path></svg></i><span>{"Loading"}</span></button>
    <button type="button" class="ant-btn ant-btn-primary ant-btn-lg ant-btn-loading"><i aria-label="icon: loading" class="anticon anticon-loading"><svg viewBox="0 0 1024 1024" focusable="false" class="anticon-spin" data-icon="loading" width="1em" height="1em" fill="currentColor" aria-hidden="true"><path d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 0 0-94.3-139.9 437.71 437.71 0 0 0-139.9-94.3C629 83.6 571.4 72 512 72c-19.9 0-36-16.1-36-36s16.1-36 36-36c69.1 0 136.2 13.5 199.3 40.3C772.3 66 827 103 874 150c47 47 83.9 101.8 109.7 162.7 26.7 63.1 40.2 130.2 40.2 199.3.1 19.9-16 36-35.9 36z"></path></svg></i><span>{"Loading"}</span></button> 
    </>
    }
}