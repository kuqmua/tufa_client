use crate::components::model::App;

pub fn entry() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
