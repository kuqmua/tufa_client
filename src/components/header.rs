use crate::components::svg::menu::Menu;
use crate::components::svg::person_outline::PersonOutline;
// use crate::routes::routes::Routes;
use yew::{function_component, html};
// use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <header
        style="
          height: 50px; 
          width: 100%;
          min-width: 470px;
          background-color: #16202A;
          border-bottom: 1px solid #1E2832;
          position: fixed;
        ">
          <div
            style="
              display: flex;
              height: 100%; 
              justify-content: space-between; 
              align-items: center;
              padding-right: 20px;
              padding-left: 20px;
            "
          >
            <div
              style="
                padding: 8px;
              "
            >
              <Menu height={"26px".to_owned()} width={"26px".to_owned()} fill={"#5B6267".to_owned()}/>
            </div>
            <div
              style="
                font-size: 30px;
                font-family: 'Koulen', cursive;
                color: white;
              "
            >
              {"Tufa Client"}
            </div>
            <div
              style="
                color: white
              "
            >
            //   <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
            //     {"----------"}
            //   <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
              <PersonOutline height={"26px".to_owned()} width={"26px".to_owned()} fill={"#5B6267".to_owned()}/>
            </div>
          </div>
        </header>
    }
}