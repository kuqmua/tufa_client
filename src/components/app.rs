use crate::components::counter::Counter;
use crate::components::display_count::DisplayCount;
use crate::components::header::Header;
use crate::routes::routes::Routes;
use crate::routes::switch::switch;
use crate::store::init;
use crate::store::YewduxStore;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

pub enum AppMessage {
    ActionOne,
}
pub struct App {
    dispatch: Dispatch<BasicStore<YewduxStore>>,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = DispatchProps<BasicStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        let dispatch = init();
        Self { dispatch }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
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
              <WithDispatch<DisplayCount>/>
              <WithDispatch<Counter>/>
              <BrowserRouter>
                  <Switch<Routes> render={Switch::render(switch)} />
              </BrowserRouter>
          </div>
        }
    }
}
