use crate::components::form::Form;
use crate::components::input_button::InputButton;
use crate::components::svg_icon_wrapper::SvgIconWrapper;
use yew::prelude::*;

// 
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(ModalFunction)]
fn modalfunction() -> Html {
  let history_option = use_history();
  let onclick: Callback<MouseEvent>;
  match history_option {
    Some(history) => onclick = Callback::once(move |_| history.push(Route::Secure)),
    None => onclick = Callback::once(move |_| log::info!("Update1")),
}
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
      class="MuiContainer-root MuiContainer-maxWidthXs"
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
        class="makeStyles-paper-1"
        style="
          display: flex;
          margin-top: 64px;
          align-items: center;
          flex-direction: column;
        "
      >
        <SvgIconWrapper />
        <h1
          class="MuiTypography-root MuiTypography-h5"
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
          class="makeStyles-form-3"
          novalidate=true
          style="
            width: 100%;
            margin-top: 24px;
            box-sizing: inherit;
            display: block;
          "
        >
          <div
            class="MuiGrid-root MuiGrid-container MuiGrid-spacing-xs-2"
            style="
              width: calc(100% + 16px);
              margin: -8px;
              display: flex;
              flex-wrap: wrap;
              box-sizing: border-box;
            "
          >
            <Form />
            <Form />
            <div
              class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12"
            >
            </div>
          </div>
          <InputButton />
          <div
            class="MuiGrid-root MuiGrid-container MuiGrid-justify-content-xs-flex-end"
            style="
              justify-content: flex-end;
              width: 100%;
              display: flex;
              flex-wrap: wrap;
              box-sizing: border-box;
            "
          >
            <div
              class="MuiGrid-root MuiGrid-item"
              style="
                margin: 0;
                box-sizing: border-box;
                display: block;
              "
            >
              <a
                class="MuiTypography-root MuiLink-root MuiLink-underlineHover MuiTypography-body2 MuiTypography-colorPrimary"
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
    <button {onclick}>{ "Go to Secure" }</button>
  </div>
 }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html!
        {
         <ModalFunction/> 
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
//
pub enum ModalMessage {
    ActionOne,
}
pub struct Model {}

impl Component for Model {
    type Message = ModalMessage;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div style="width: 100%; display: flex; justify-content: center; flex-direction: column; align-items: center;">
          <div style="height: 50px; width: 100%; background-color: yellow; display: flex; justify-content: center; align-items: center;"><h1>{ "Hi!" }</h1></div>
          <BrowserRouter>
          <Switch<Route> render={Switch::render(switch)} />
      </BrowserRouter>
          </div>
        }
    }
}
