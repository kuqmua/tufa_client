use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let onchange = Callback::from(|event: Event| {
        let target = event.target().unwrap();
        log!(target);
    });
    html! {
        <input type="text" name={props.name.clone()} onchange={onchange}/>
    }
}
