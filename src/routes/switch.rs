use crate::components::secure::Secure;
use crate::components::{auth_modal::AuthModal, secure::Color};
use crate::routes::routes::Routes;
use yew::{html, Html};

pub fn switch(routes: &Routes) -> Html {
    match routes {
        Routes::Home => html! {
         <AuthModal/>
        },
        Routes::Secure => html! {
            <Secure first="my_first_prop" color={Color::Ok}/>
        },
        Routes::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
