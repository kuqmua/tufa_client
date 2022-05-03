use crate::components::header::Header;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        // log!(target.clone());
        // log!(target.value_of());
        // log!(input.value());
        handle_onchange.emit(value);
    });
    html! {
        <div>
            <input type="text" name={props.name.clone()} onchange={onchange}/>
           <Header/>
        </div>

    }
}
