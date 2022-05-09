use crate::store::YewduxStore;
use gloo::console::log;
use lazy_static::__Deref;
use reqwasm::http::Request;
use serde_json::{from_str, json};
use tufa_common::json_example::JsonExample;
use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::use_store;

#[derive(Default, Clone, Debug)]
pub struct GetDataFromServerButtonState {
    username: String,
}

#[function_component(PostDataToServerButton)]
pub fn post_data_to_server_button() -> Html {
    let state = use_state(GetDataFromServerButtonState::default);
    let cloned_state = state.clone();
    let store = use_store::<PersistentStore<YewduxStore>>();
    let onclick: Callback<MouseEvent> = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let json = json!({
                    "first": "first_from_tufa_client".to_string(),
                    "second": "second_from_tufa_client".to_string(),
                });
                let f = Request::post("http://127.0.0.1:8081/api/json/json_example_post")
                    .header("content-type", "application/json")
                    .body(json.to_string())
                    .send()
                    .await
                    .unwrap()
                    // .json::<JsonExample>()
                    // .await
                    // .unwrap()
                    ;
                println!("f {:#?}", f.body());
                // match f {
                //     Ok(k) => {
                //         match k.text().await {
                //             Ok(n) => {
                //                 log!(format!("ok {:#?}", n));
                //                 let json: Result<JsonExample, serde_json::Error> = from_str(&n);
                //                 match json {
                //                     Ok(l) => {}
                //                     Err(e) => log!(format!("2err {:#?}", e)),
                //                 }
                //             }
                //             Err(e) => log!(format!("1err {:#?}", e)),
                //         }
                //         log!("ok {:#?}", k.body());
                //     }
                //     Err(_) => log!("0err"),
                // }
                log!("Update2:");
            });
        })
    };
    html! {
      <button onclick={onclick}>{"post json example"}</button>
    }
}
