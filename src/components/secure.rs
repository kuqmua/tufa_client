use crate::routes::routes::Routes;
use stylist::yew::styled_component;
use stylist::{style, Style};
use yew::{html, Callback, Html, Properties};
use yew_router::hooks::use_history;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SecureProps {
    pub first: String,
    pub color: Color,
}

const STYLE_FILE: &str = include_str!("example.css");

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match &self {
            Color::Normal => "normal".to_string(),
            Color::Ok => "ok".to_string(),
            Color::Error => "error".to_string(),
        }
    }
}

#[styled_component(Secure)]
pub fn secure(props: &SecureProps) -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Routes::Home));
    let class = "my_title";
    let option: Option<&str> = Some("kekw");
    let example_list = vec!["one", "two", "three"];
    let example_list_html = vec![
        html! { <div>{"one"}</div>},
        html! { <div>{"two"}</div>},
        html! { <div>{"three"}</div>},
    ];
    let example_list_for_ter = vec!["oneiter", "twoiter", "threeiter"];
    let example_list_for_function = vec!["one_function", "two_function", "three_function"];

    let stylesheet = style!(
        r#"
            background-color: blue;
        "#
    )
    .unwrap();
    let file_stylesheet = Style::new(STYLE_FILE).unwrap();
    //<h1 class={file_stylesheet}>{"file_stylesheet"}</h1>
    html! {
        <div class={file_stylesheet}>
        {"file_stylesheet"}
            <div class={stylesheet}>{"stylesheet"}</div>
            if class == "my_title" {
                <h1 class={class}>{ "my_title" }</h1>
            }
            else {
                <h1 class={class}>{ "not my_title" }</h1>
            }
            if let Some(option) = option {
                <h1 class={class}>{option}</h1>
            }
            else  {
                <h1 class={class}>{"None"}</h1>
            }
            <h1 class={css!("color: red; background-color: green;")}>{"css! macro"}</h1>
            <h1>{&props.first}</h1>
            <h1>{&props.color.to_string()}</h1>
            {example_list}
            {example_list_html}
            {example_list_for_ter.iter().map(|x| html!{<li>{"iter of"}{x}</li>}).collect::<Html>()}
            {list_to_html(example_list_for_function)}
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

pub fn list_to_html(vec: Vec<&str>) -> Vec<Html> {
    vec.iter().map(|x| html! {<li>{x}</li>}).collect()
}
