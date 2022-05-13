use yew::{function_component, html};
use yew_router::prelude::Link;
use crate::routes::routes::Routes;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div 
            style="
                height: 50px; 
                width: 100%; 
                background-color: yellow; 
                display: flex; 
                justify-content: space-between; 
                align-items: center;
            ">
            <style>
               {"body { 
                   background-color: #16202A;
                   margin: 0px;
                   padding: 0px;
                }"}
           </style>
            <Link<Routes> to={Routes::Home}>{ "home" }</Link<Routes>>
            <div>
                <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
                {"----------"}
                <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
            </div>
        </div>
    }
}
