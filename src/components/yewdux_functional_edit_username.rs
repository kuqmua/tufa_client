use crate::store::YewduxStore;
use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

#[function_component(YewduxFunctionalEditUsername)]
pub fn yewdux_functional_edit_username() -> Html {
    // let handle_username_change =
    //     ctx.props()
    //         .dispatch()
    //         .reduce_callback_with(|state, event: Event| {
    //             let username = event
    //                 .target()
    //                 .unwrap()
    //                 .unchecked_into::<HtmlInputElement>()
    //                 .value();
    //             state.username = username;
    //             log!("username", state.username.clone());
    //         });
    html! {
      <form>
        <h1>{"Login"}</h1>
        <div>
          <input type="text" placeholder="username" onchange={|e| log!("test")}/>
        </div>
        <div>
          <button>{"Log in"}</button>
        </div>
      </form>
    }
}
