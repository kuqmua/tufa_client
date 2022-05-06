use crate::components::header::Header;
use crate::routes::routes::Routes;
use crate::routes::switch::switch;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

pub enum AppMessage {
    ActionOne,
}
pub struct App {}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        //     <style>
        //     body {
        //         padding: 0px;
        //         margin: 0px;
        //     }
        // </style>
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
              <BrowserRouter>
                  <Switch<Routes> render={Switch::render(switch)} />
              </BrowserRouter>
          </div>
        }
    }
}
