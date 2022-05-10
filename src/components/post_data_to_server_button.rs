use gloo::console::log;
use reqwasm::http::Request;
use serde_json::json;
use yew::prelude::*;

#[function_component(PostDataToServerButton)]
pub fn post_data_to_server_button() -> Html {
    let onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::post("http://127.0.0.1:8081/api/json/json_example_post")
                    .header("content-type", "application/json")
                    .body(
                        json!({
                            "first": "first_from_tufa_client".to_string(),
                            "second": "second_from_tufa_client".to_string(),
                        })
                        .to_string(),
                    )
                    .send()
                    .await
                {
                    Err(e) => log!("error 57435634753434 ", e.to_string()),
                    Ok(f) => {
                        let text = f.text().await;
                        match text {
                            Err(e) => log!("error 46396426462 ", e.to_string()),
                            Ok(t) => log!("f ", t),
                        }
                    }
                }
            });
        })
    };
    html! {
      <button onclick={onclick}>{"post json example"}</button>
    }
}
