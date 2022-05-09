use crate::components::counter::Counter;
use crate::components::display_count::DisplayCount;
use crate::components::get_data_from_server_button::GetDataFromServerButton;
use crate::components::header::Header;
use crate::components::yewdux_functional_component_example::YewduxFunctionalComponentExample;
use crate::routes::routes::Routes;
use crate::routes::switch::switch;
use crate::store::init;
use crate::store::YewduxStore;
// use wasm_bindgen::JsCast;
// use web_sys::HtmlInputElement;
use gloo::console::log;
use lazy_static::__Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

// use gloo::console::log;
use reqwasm::http::Request;
use serde_json::from_str;
use tufa_common::json_example::JsonExample;
use wasm_bindgen::JsCast;

pub enum AppMessage {
    ActionOne,
}
pub struct App {
    pub dispatch: Dispatch<PersistentStore<YewduxStore>>,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        let dispatch = init();
        Self { dispatch }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        // let handle_form_submit = Callback::from(|event: FocusEvent| {
        //     event.prevent_default();
        // });
        // let handle_username_change =
        //     ctx.props()
        //         .dispatch()
        //         .reduce_callback_with(|state, event: Event| {
        //             let username = event
        //                 .target()
        //                 .unwrap()
        //                 .unchecked_into::<HtmlInputElement>()
        //                 .value();
        //             state.username = username;
        //         });
        // let onclick = Callback::from(move |_| {
        //     wasm_bindgen_futures::spawn_local(async move {
        //         log!("Update1");
        //         let f = Request::get("http://127.0.0.1:8081/api/json/json_example")
        //             .send()
        //             .await;
        //         match f {
        //             Ok(k) => {
        //                 match k.text().await {
        //                     Ok(n) => {
        //                         log!(format!("ok {:#?}", n));
        //                         let json: Result<JsonExample, serde_json::Error> = from_str(&n);
        //                         match json {
        //                             Ok(_l) => {
        //                                 // let mut bbb = l.second.clone();
        //                                 // self.some_string = "ertrer".to_string();
        //                                 // self.set_second(bbb);
        //                                 // log!("ok {:#?}", l);
        //                             }
        //                             Err(e) => log!(format!("2err {:#?}", e)),
        //                         }
        //                     }
        //                     Err(e) => log!(format!("1err {:#?}", e)),
        //                 }
        //                 log!("ok {:#?}", k.body());
        //             }
        //             Err(_) => log!("0err"),
        //         }
        //         log!("Update2:");
        //     });
        // });
        // let handle_username_change =
        //     ctx.props()
        //         .dispatch()
        //         .reduce_callback_with(|state, event: Event| {
        //             // let username = event
        //             //     .target()
        //             //     .unwrap()
        //             //     .unchecked_into::<HtmlInputElement>()
        //             //     .value();
        //             // state.username = username;
        //             // log!("username", state.username.clone());
        //             // let mut nnn = String::from("yyey");
        //             let mut f: JsonExample = JsonExample {
        //                 first: String::from("111"),
        //                 second: String::from("222"),
        //             };
        //             wasm_bindgen_futures::spawn_local(async move {
        //                 log!("Update1");
        //                 let b = Request::get("http://127.0.0.1:8081/api/json/json_example")
        //                     .send()
        //                     .await
        //                     .unwrap()
        //                     .json::<JsonExample>()
        //                     .await
        //                     .unwrap();
        //                 let mut state_cloned = state.deref().clone();
        //                 state_cloned.username = b.second;
        //                 // state.set(state_cloned);
        //                 state = &mut state_cloned;
        //                 // f = b;
        //                 // return b;
        //                 // state.username =  f.second;
        //                 // match f {
        //                 //     Ok(k) => {
        //                 //         match k.text().await {
        //                 //             Ok(n) => {
        //                 //                 log!(format!("ok {:#?}", n));
        //                 //                 let json: Result<JsonExample, serde_json::Error> =
        //                 //                     from_str(&n);
        //                 //                 match json {
        //                 //                     Ok(l) => {
        //                 //                         // nnn = l.second.clone();
        //                 //                         f = JsonExample {
        //                 //                             first: l.first,
        //                 //                             second: l.second,
        //                 //                         };
        //                 //                         // let mut bbb = l.second.clone();
        //                 //                         // self.some_string = "ertrer".to_string();
        //                 //                         // self.set_second(bbb);
        //                 //                         // log!("ok {:#?}", l);
        //                 //                     }
        //                 //                     Err(e) => log!(format!("2err {:#?}", e)),
        //                 //                 }
        //                 //             }
        //                 //             Err(e) => log!(format!("1err {:#?}", e)),
        //                 //         }
        //                 //         log!("ok {:#?}", k.body());
        //                 //     }
        //                 //     Err(_) => log!("0err"),
        //                 // }
        //                 // log!("Update2:");
        //             });
        //             state.username = f.second;
        //         });
        html! {
          <div
              style="
                  width: 100%; 
                  display: flex; 
                  justify-content: center; 
                  flex-direction: column; 
                  align-items: center;
              "
          >

              <Header/>
            //   <button onclick={get_message}>{"click me to fetch data from server"}</button>
            //   <button onclick={onclick}>{"click me to fetch data from server"}</button>
              <GetDataFromServerButton/>
              <YewduxFunctionalComponentExample/>
              <WithDispatch<DisplayCount>/>
              <WithDispatch<Counter>/>
              <BrowserRouter>
                  <Switch<Routes> render={Switch::render(switch)} />
              </BrowserRouter>
          </div>
        }
    }
}
