use yew::html;
use yew::prelude::*;
use yew::Html;

pub struct Example {}

impl Component for Example {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div
            style=""
          >
          </div>
        }
    }
}
