use crate::components::authorization::sign_up::component::SignUp;
use crate::components::authorization::sign_in::component::SignIn;
use crate::components::examples::counter::Counter;
use crate::components::home::Home;
use crate::components::examples::secure::Color;
use crate::components::examples::secure::Secure;
use crate::components::examples::secure::SecureProps;
use crate::components::examples::example::Example;
use crate::components::examples::markdown_like::MarkdownLike;
use crate::routes::routes::Routes;
use gloo::console::log;
use yew::{html, Callback, Html};
use yew_router::prelude::Link;
use yewdux::prelude::*;

pub fn switch_routes(routes: &Routes) -> Html {
    let main_title_head: Callback<String> = Callback::from(|message| log!(message));
    let custom_from_submit = Callback::from(|data: SecureProps| log!("first is", data.first));
    match routes {
        Routes::Home => html! {
            // <WithDispatch<Home>/>
            <Home/>
        },
        Routes::Markdown => html! {
            <MarkdownLike/>
        },
        Routes::SignUp => html! {
            // <WithDispatch<SignUp>/>
            <SignUp/>
        },
        Routes::SignIn => html! {
            // <WithDispatch<SignIn>/>
            <SignIn/>
        },
        Routes::Secure => html! {
            <Secure first="my_first_prop" color={Color::Ok} on_load={main_title_head} onsubmit={custom_from_submit}/>
        },
        Routes::NotFound => html! {
           <div>
              <h1>{ "404" }</h1>
              <Link<Routes> to={Routes::SignUp}>{ "click here to go to sign up" }</Link<Routes>>
           </div>
        },
        Routes::CounterHandle => html! { <WithDispatch<Counter>/> },
        Routes::Example => html! { <WithDispatch<Example>/> },
    }
}
