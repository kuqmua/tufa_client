use yew::{function_component, html};
use yew_router::prelude::Link;
use crate::routes::routes::Routes;

#[function_component(Header)]
pub fn header() -> Html {
    // let history = use_history().unwrap();
    // let onclick = Callback::once(move |_| history.push(Routes::Home));
    html! {
        <div style="height: 50px; width: 100%; background-color: yellow; display: flex; justify-content: space-between; align-items: center;">
            <Link<Routes> to={Routes::Home}>{ "home" }</Link<Routes>>
            <div>
                <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
                {"----------"}
                <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
            </div>
        </div>
    }
}
