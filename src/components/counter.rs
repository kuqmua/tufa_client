// use crate::components::header::Header;
// use crate::routes::routes::Routes;
// use crate::routes::switch::switch;
use yew::prelude::*;
// use yew_router::prelude::*;
use yewdux::prelude::*;

pub enum CounterMessage {
    ActionOne,
}
pub struct Counter;

impl Component for Counter {
    type Message = ();
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div>
            <h1>{format!("counter pressed {} times", 0)}</h1>
          </div>
        }
    }
}
