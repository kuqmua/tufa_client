use gloo::console::log;
use reqwasm::http::Request;
use serde_json::from_str;
use tufa_common::json_example::JsonExample;
use yew::prelude::*;

pub struct SubmitButton {}

impl Component for SubmitButton {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        wasm_bindgen_futures::spawn_local(async move {
            log!("Update1");
            let f = Request::get("http://127.0.0.1:8081/api/json/json_example")
                .send()
                .await;
            match f {
                Ok(k) => {
                    match k.text().await {
                        Ok(n) => {
                            log!(format!("ok {:#?}", n));
                            let json: Result<JsonExample, serde_json::Error> = from_str(&n);
                            match json {
                                Ok(_l) => {
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
        // the value has changed so we need to
        // re-render for it to appear on the page
        true
    }
    //https://codepen.io/shawnc8160/pen/xxRYOWg
    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        //for some reason page re renders if it would be button
        html! {
          <span
            onclick={link.callback(|_| log!("click"))}
            tabindex="0"
            type="submit"
            style="
              -webkit-font-smoothing: antialiased;
              border: 0;
              cursor: pointer;
              display: inline-flex;
              outline: 0;
              position: relative;
              align-items: center;
              user-select: none;
              vertical-align: middle;
              justify-content: center;
              text-decoration: none;
              -webkit-appearance: none;
              -webkit-tap-highlight-color: transparent;
              padding: 6px 16px;
              font-size: 0.875rem;
              min-width: 64px;
              box-sizing: border-box;
              transition: background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,border 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
              font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
              font-weight: 500;
              line-height: 1.75;
              border-radius: 4px;
              letter-spacing: 0.02857em;
              text-transform: uppercase;
              box-shadow: 0px 3px 1px -2px rgba(0,0,0,0.2),0px 2px 2px 0px rgba(0,0,0,0.14),0px 1px 5px 0px rgba(0,0,0,0.12);
              color: #fff;
              background-color: #556cd6;
              width: 100%;
              margin: 24px 0px 16px;
            "
          >
            <span
              style="
                -webkit-font-smoothing: antialiased;
                cursor: pointer;
                user-select: none;
                -webkit-tap-highlight-color: transparent;
                font-size: 0.875rem;
                font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                font-weight: 500;
                line-height: 1.75;
                letter-spacing: 0.02857em;
                text-transform: uppercase;
                color: #fff;
                box-sizing: inherit;
                width: 100%;
                display: inherit;
                align-items: inherit;
                justify-content: inherit;
              "
            >
              {"Sign Up"}
            </span>
          </span>
        }
    }
}
