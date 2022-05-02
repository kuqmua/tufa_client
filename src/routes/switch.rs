use crate::components::secure::Secure;
use crate::components::{auth_modal::AuthModal, secure::Color};
use crate::routes::routes::Routes;
use gloo::console::log;
use yew::{html, Callback, Html};

pub fn switch(routes: &Routes) -> Html {
    let main_title_head: Callback<String> = Callback::from(|message| log!(message));
    match routes {
        Routes::Home => html! {
         <AuthModal/>
        },
        Routes::Secure => html! {
            <Secure first="my_first_prop" color={Color::Ok} on_load={main_title_head}/>
        },
        Routes::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
