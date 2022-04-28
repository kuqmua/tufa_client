use yew::{Html, html};
use crate::components::model::AuthModal;
use crate::routes::routes::Routes;
use crate::components::secure::Secure;

pub fn switch(routes: &Routes) -> Html {
    match routes {
        Routes::Home => html!
        {
         <AuthModal/> 
        },
        Routes::Secure => html! {
            <Secure />
        },
        Routes::NotFound => html! { <h1>{ "404" }</h1> },
    }
}