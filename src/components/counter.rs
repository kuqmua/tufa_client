// use crate::components::header::Header;
// use crate::routes::routes::Routes;
// use crate::routes::switch::switch;
use yew::prelude::*;
// use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

pub enum CounterMessage {
    ActionOne,
}
pub struct Counter;

impl Component for Counter {
    type Message = ();
    type Properties = DispatchProps<BasicStore<YewduxStore>>;
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;
        let onclick = ctx
            .props()
            .dispatch()
            .reduce_callback(|state| state.count += 1);
        // .reduce_callback_with(|state, event| state.count += 1);
        html! {
          <div>
            <button onclick={onclick}>{"click me"}</button>
          </div>
        }
    }
}
