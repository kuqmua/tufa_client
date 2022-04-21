use crate::components::model::Model;

pub fn entry() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
