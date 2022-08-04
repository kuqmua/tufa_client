use gloo::console::log;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::html::onchange::Event;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::Properties;
use yew::UseStateHandle;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct SelectProps {
    pub values: Vec<String>,
    pub default_value: String,
    pub style: Option<String>,
    pub additional_classes: Option<String>,
    pub set_choosen_value: Callback<(Event, String)>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let is_open = use_state(|| false);
    let choosen_value: UseStateHandle<String> = use_state(|| props.default_value.clone());
    let choosen_value_cloned = choosen_value.clone();
    let choosen_value_second_cloned = choosen_value.clone();
    let set_choosen_value_cloned = props.set_choosen_value.clone();
    let on_open = Callback::<Event>::from(move |e: Event| {
        // e.prevent_default();
        log!("wtf");
        is_open.set(!*is_open.clone());
        let value = &*choosen_value_second_cloned.clone();
        set_choosen_value_cloned.emit((e, value.to_string()));
    });
    let on_click = Callback::<MouseEvent>::from(move |e: MouseEvent| {
        log!("oooo");
    });
    let options = props
        .values
        .iter()
        .enumerate()
        .map(|(index, v)| {
            if v.clone() == *choosen_value_cloned.clone() {
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
        onchange={on_open.clone()}
        class={classes}
        style={style}
      >
        {options}
      </select>
    }
}
