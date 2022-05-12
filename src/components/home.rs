use crate::{store::YewduxStore};
use yew::prelude::*;
use yewdux::prelude::DispatchProps;
use yewdux::prelude::PersistentStore;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
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
          <div>{"home"}</div>
        }
    }
}
