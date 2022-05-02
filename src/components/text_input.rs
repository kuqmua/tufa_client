use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    html! {
        <input type="text" name={props.name.clone()}/>
    }
}
