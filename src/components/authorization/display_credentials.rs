use crate::store::YewduxStore;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct DisplayCredentials;

impl Component for DisplayCredentials {
    type Message = ();
    type Properties = DispatchProps<BasicStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let username = ctx.props().state().username.clone();
        let password = ctx.props().state().password.clone();
        html! {
          <div>
            <h1>{"Display username"}</h1>
            <p>{format!("username: {}", username)}</p>
            <h1>{"Display password"}</h1>
            <p>{format!("password: {}", password)}</p>
          </div>
        }
    }
}
