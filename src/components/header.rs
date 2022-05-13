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
            <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0z" fill="none"/><path d="M3 15h18v-2H3v2zm0 4h18v-2H3v2zm0-8h18V9H3v2zm0-6v2h18V5H3z"/></svg>
            <Link<Routes> to={Routes::Home}>{ "home" }</Link<Routes>>
            <div>
                <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
                {"----------"}
                <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
            </div>
        </div>
    }
}
