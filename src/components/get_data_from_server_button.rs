use crate::store::YewduxStore;
use gloo::console::log;
use lazy_static::__Deref;
use reqwasm::http::Request;
use serde_json::from_str;
use tufa_common::json_example::JsonExample;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::use_store;

#[derive(Default, Clone, Debug)]
pub struct GetDataFromServerButtonState {
    username: String,
}

#[function_component(GetDataFromServerButton)]
pub fn get_data_from_server_button() -> Html {
    let state = use_state(|| GetDataFromServerButtonState::default());
    let cloned_state = state.clone();
    let store = use_store::<PersistentStore<YewduxStore>>();
    let handle_form_submit =
        store
            .dispatch()
            .reduce_callback_with(move |yewduxstore, event: FocusEvent| {
                event.prevent_default();
                yewduxstore.username = cloned_state.username.clone();
                // log!("handle_form_submit username", state.username.clone());
            });
    let onclick: Callback<MouseEvent> = {
        let state = state.clone();
        Callback::from(move |event: MouseEvent| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log!("Update1");
                let f = Request::get("http://127.0.0.1:8081/api/json/json_example")
                    .send()
                    .await;
                // .unwrap()
                // .json::<JsonExample>()
                // .await
                // .unwrap();
                // state.username = b.second;

                match f {
                    Ok(k) => {
                        match k.text().await {
                            Ok(n) => {
                                log!(format!("ok {:#?}", n));
                                let json: Result<JsonExample, serde_json::Error> = from_str(&n);
                                match json {
                                    Ok(l) => {
                                        let mut state_cloned = state.deref().clone();
                                        state_cloned.username = l.second;
                                        state.set(state_cloned);
                                        // // nnn = l.second.clone();
                                        // f = JsonExample {
                                        //     first: l.first,
                                        //     second: l.second,
                                        // };
                                        // let mut bbb = l.second.clone();
                                        // self.some_string = "ertrer".to_string();
                                        // self.set_second(bbb);
                                        // log!("ok {:#?}", l);
                                    }
                                    Err(e) => log!(format!("2err {:#?}", e)),
                                }
                            }
                            Err(e) => log!(format!("1err {:#?}", e)),
                        }
                        log!("ok {:#?}", k.body());
                    }
                    Err(_) => log!("0err"),
                }
                log!("Update2:");
            });
        })
    };
    let username = store
        .state()
        .map(|state| state.username.clone())
        .unwrap_or_default();
    // let handle_username_change = store.dispatch().reduce_callback_with(|yewduxstore, event| {

    //     // state.username = username;
    // });
    // let onclick = {
    //     let state = state.clone();
    //     Callback::from(move |_| {
    //         let state = state.clone();

    //     });
    // };
    // let handle_username_change = store.dispatch().reduce_callback_with(|state, event| {
    //     wasm_bindgen_futures::spawn_local(async move {
    //         log!("Update1");
    //         let b = Request::get("http://127.0.0.1:8081/api/json/json_example")
    //             .send()
    //             .await
    //             .unwrap()
    //             .json::<JsonExample>()
    //             .await
    //             .unwrap();
    //         let mut state_cloned = state.deref().clone();
    //         state_cloned.username = b.second;
    //         // state.set(state_cloned);
    //         // state = &mut state_cloned;
    //         // f = b;
    //         // return b;
    //         // state.username =  f.second;
    //         // match f {
    //         //     Ok(k) => {
    //         //         match k.text().await {
    //         //             Ok(n) => {
    //         //                 log!(format!("ok {:#?}", n));
    //         //                 let json: Result<JsonExample, serde_json::Error> =
    //         //                     from_str(&n);
    //         //                 match json {
    //         //                     Ok(l) => {
    //         //                         // nnn = l.second.clone();
    //         //                         f = JsonExample {
    //         //                             first: l.first,
    //         //                             second: l.second,
    //         //                         };
    //         //                         // let mut bbb = l.second.clone();
    //         //                         // self.some_string = "ertrer".to_string();
    //         //                         // self.set_second(bbb);
    //         //                         // log!("ok {:#?}", l);
    //         //                     }
    //         //                     Err(e) => log!(format!("2err {:#?}", e)),
    //         //                 }
    //         //             }
    //         //             Err(e) => log!(format!("1err {:#?}", e)),
    //         //         }
    //         //         log!("ok {:#?}", k.body());
    //         //     }
    //         //     Err(_) => log!("0err"),
    //         // }
    //         // log!("Update2:");
    //     });
    //     // state.username = f.second;
    // });
    // let handle_username_change =
    //     ctx.props()
    //         .dispatch()
    //         .reduce_callback_with(|state, event: Event| {

    //         });

    html! {
        <div>
              <button onclick={onclick}>{"Log in"}</button>
              <div>{format!("!{}!", state.username.clone())}</div>
                    <form onsubmit={handle_form_submit}>

        <div>
          <button>{"bif g"}</button>
        </div>
      </form>
              <div>{"username is"}</div>
        <div>{username}</div>
        </div>




    }
}
