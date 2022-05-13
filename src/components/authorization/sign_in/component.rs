use crate::helpers::html_input_type::HtmlInputType;
use crate::components::authorization::input_form::InputForm;
use crate::components::svg_icon_wrapper::SvgIconWrapper;
use crate::routes::routes::Routes;
use crate::{components::authorization::submit_button::SubmitButton, store::YewduxStore};
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::DispatchProps;
use yewdux::prelude::Dispatcher;
use yewdux::prelude::PersistentStore;
use yewdux::prelude::WithDispatchProps;

pub struct SignIn {
    pub dispatch: DispatchProps<PersistentStore<YewduxStore>>,
    pub header_name: String,
}

impl Component for SignIn {
    type Message = ();
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
    fn create(ctx: &Context<Self>) -> Self {
        let _dispatch = ctx.props().dispatch().clone();
        // let header_name = ctx.props().header_name;
        Self {
            dispatch: _dispatch,
            header_name: String::from("Sign in")
        }
    }
    // fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    //     // true
    //     false
    // }
    // fn changed(&mut self, _ctx: &Context<Self>) -> bool {
    //     // true
    //     false
    // }
    // fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    // fn destroy(&mut self, _ctx: &Context<Self>) {}
    fn view(&self, ctx: &Context<Self>) -> Html {
        let handle_form_submit = {
            let ctx = ctx.props().state();
            Callback::from(move |event: FocusEvent| {
                event.prevent_default();
                let username = ctx.username.clone();
                let password = ctx.password.clone();
                log!("Username: ", username, "Password: ", password);
            })
        };
        let handle_username_change =
            ctx.props()
                .dispatch()
                .reduce_callback_with(|state, event: Event| {
                    let username = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>()
                        .value();
                    state.username = username;
                    log!("username", state.username.clone());
                });
        let handle_password_change =
            ctx.props()
                .dispatch()
                .reduce_callback_with(|state, event: Event| {
                    let password = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>()
                        .value();
                    state.password = password;
                    log!("password", state.password.clone());
                });
        html! {
          <div
            id="root"
            style="
              display: block;
              color: rgba(0, 53, 0, 0.87);
              margin: 0;
              font-size: 0.875rem;
              font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
              font-weight: 400;
              line-height: 1.43;
              letter-spacing: 0.01071em;
              background-color: #fff;
              -webkit-font-smoothing: antialiased;
              box-sizing: inherit;
            "
          >
            <div
              style="
                max-width: 444px;
                width: 100%;
                display: block;
                box-sizing: border-box;
                margin-left: auto;
                margin-right: auto;
                padding-left: 16px;
                padding-right: 16px;
              "
            >
              <div
                style="
                  display: flex;
                  margin-top: 64px;
                  align-items: center;
                  flex-direction: column;
                "
              >
                <SvgIconWrapper/>
                <h1
                  style="
                    font-size: 1.5rem;
                    font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                    font-weight: 400;
                    line-height: 1.334;
                    letter-spacing: 0em;
                    margin: 0;
                    box-sizing: inherit;
                    display: block;
                    font-size: 2em;
                    margin-block-start: 0.67em;
                    margin-block-end: 0.67em;
                    margin-inline-start: 0px;
                    margin-inline-end: 0px;
                    font-weight: bold;
                  "
                >
                  {self.header_name.clone()}
                </h1>
                <form
                  onsubmit={handle_form_submit.clone()}
                  novalidate=true
                  style="
                    width: 100%;
                    margin-top: 24px;
                    box-sizing: inherit;
                    display: block;
                  "
                >
                  <div
                    style="
                      width: calc(100% + 16px);
                      margin: -8px;
                      display: flex;
                      flex-wrap: wrap;
                      box-sizing: border-box;
                    "
                  >
                    <InputForm placeholder={"Login".to_owned()} input_type={HtmlInputType::Text} action={handle_username_change} />
                    <InputForm placeholder={"Password".to_owned()} input_type={HtmlInputType::Password} action={handle_password_change} />
                  </div>
                  <div>
                    <SubmitButton action={handle_form_submit} placeholder={self.header_name.clone()} />
                  </div>
                  <div
                    style="
                      justify-content: flex-end;
                      width: 100%;
                      display: flex;
                      flex-wrap: wrap;
                      box-sizing: border-box;
                    "
                  >
                    <div
                      style="
                        margin: 0;
                        box-sizing: border-box;
                        display: block;
                      "
                    >
                      <a
                        href=""
                        style="
                          text-decoration: none;
                          color: #556cd6;
                          font-size: 0.875rem;
                          font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                          font-weight: 400;
                          line-height: 1.43;
                          letter-spacing: 0.01071em;
                          margin: 0;
                          box-sizing: inherit;
                          cursor: pointer;
                        "
                      >
                      {"Don't have an account? "}<Link<Routes> to={Routes::SignUp}>{ "Sign up" }</Link<Routes>>
                      </a>
                    </div>
                  </div>
                </form>
              </div>
            </div>
            <Link<Routes> to={Routes::Secure}>{ "Go to Secure" }</Link<Routes>>
          </div>
        }
    }
}
