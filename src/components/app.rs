use crate::components::examples::counter::Counter;
use crate::components::examples::display_count::DisplayCount;
use crate::components::examples::get_data_from_server_button::GetDataFromServerButton;
use crate::components::header::Header;
use crate::components::examples::post_data_to_server_button::PostDataToServerButton;
use crate::components::examples::yewdux_functional_component_example::YewduxFunctionalComponentExample;
use crate::components::examples::set_timeout_example::SetTimeoutExample;
use crate::components::examples::without_html_tag_example::WithoutHtmlTagExample;
use crate::routes::routes::Routes;
use crate::routes::switch_routes::switch_routes;
use crate::store::init;
use crate::store::YewduxStore;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

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
              <WithoutHtmlTagExample/>
              <SetTimeoutExample/>
              <PostDataToServerButton/>
              <GetDataFromServerButton/>
              <YewduxFunctionalComponentExample/>
              <WithDispatch<DisplayCount>/>
              <WithDispatch<Counter>/>
              <BrowserRouter>
                  <Switch<Routes> render={Switch::render(switch_routes)} />
              </BrowserRouter>
          </div>
        }
    }
}
