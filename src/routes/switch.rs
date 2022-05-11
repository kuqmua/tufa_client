use crate::components::authorization::auth_modal::AuthModal;
use crate::components::examples::counter::Counter;
use crate::components::secure::Color;
use crate::components::secure::Secure;
use crate::components::secure::SecureProps;
use crate::routes::routes::Routes;
use gloo::console::log;
use yew::{html, Callback, Html};
use yew_router::prelude::Link;
use yewdux::prelude::*;

pub fn switch(routes: &Routes) -> Html {
    let main_title_head: Callback<String> = Callback::from(|message| log!(message));
    let custom_from_submit = Callback::from(|data: SecureProps| log!("first is", data.first));
    match routes {
        Routes::Home => html! {
            <WithDispatch<AuthModal>/>
        },
        Routes::Secure => html! {
            <Secure first="my_first_prop" color={Color::Ok} on_load={main_title_head} onsubmit={custom_from_submit}/>
        },
        Routes::NotFound => html! {
           <div>
              <h1>{ "404" }</h1>
              <Link<Routes> to={Routes::Home}>{ "click here to go to home" }</Link<Routes>>
           </div>
        },
        Routes::CounterHandle => html! { <WithDispatch<Counter>/> },
    }
}
