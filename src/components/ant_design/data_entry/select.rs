use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::Properties;
use yew::UseStateHandle;

#[derive(Debug, PartialEq, Eq, Properties, Clone)]
pub struct SelectProps {
    pub values: Vec<String>,
    pub default_value: String,
    pub style: Option<String>,
    pub additional_classes: Option<String>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let is_open = use_state(|| false);
    let choosen_value: UseStateHandle<Option<String>> = use_state(|| None);
    let on_click = Callback::<MouseEvent>::from(move |e: MouseEvent| {
        e.prevent_default();
        is_open.set(!*is_open);
    });
    let choosen_text = match &*choosen_value {
        None => props.default_value.clone(),
        Some(cv) => cv.to_string(),
    };
    let options = props
        .values
        .iter()
        .enumerate()
        .map(|(index, v)| {
            if v.clone() == choosen_text {
                html! {
                    <option selected={true} value={index.to_string()}>{v}</option>
                }
            } else {
                html! {
                    <option value={index.to_string()}>{v}</option>
                }
            }
        })
        .collect::<Html>();
    let style = match props.style.clone() {
        None => String::from(""),
        Some(s) => s,
    };
    let classes = match props.additional_classes.clone() {
        None => String::from("form-select"),
        Some(ac) => format!("form-select {}", ac),
    };
    html! {
      <select
        onclick={on_click}
        class={classes}
        style={style}
      >
        {options}
      </select>
    }
}
