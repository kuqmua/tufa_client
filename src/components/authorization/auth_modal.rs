use crate::components::authorization::input_button::InputButton;
use crate::components::authorization::input_form::InputForm;
use crate::components::svg_icon_wrapper::SvgIconWrapper;
use crate::routes::routes::Routes;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct AuthModal {}

impl Component for AuthModal {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div
            id="root"
            style="
              display: block;
              color: rgba(0, 0, 0, 0.87);
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
                {"Sign up"}
              </h1>
              <form
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
                  <InputForm message_handle={"Login".to_owned()}/>
                  <InputForm message_handle={"Password".to_owned()}/>
                </div>
                <InputButton />
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
                      {"Already have an account? Sign in"}
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
