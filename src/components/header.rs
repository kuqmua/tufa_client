use crate::components::svg::menu::Menu;
use crate::components::svg::person_outline::PersonOutline;
use crate::components::svg::logout::Logout;
// use crate::routes::routes::Routes;
use yew::{function_component, html};
// use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <header
        style="
          width: 100%;
          min-width: 250px;
          background-color: #16202A;
          position: fixed;
          display: flex;
          flex-direction: column;
        ">
          <div
            style="
              height: 42px; 
              border-bottom: 1px solid #222c36;
              display: flex;
              flex-direction: column;
            "
          >
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
                  display: flex;
                  justify-content: center;
                  align-items: center;
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
                  display: flex;
                  justify-content: center;
                  align-items: center;
                "
              >
          //   <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
          //     {"----------"}
          //   <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
                <PersonOutline height={"26px".to_owned()} width={"26px".to_owned()} fill={"#5B6267".to_owned()}/>
              </div>
            </div>
          </div>
          <div
            style="
              position: absolute;
              top: 43px;
              right: 0px;
              height: 400px;
              width: 190px;
              border-radius: 0px 0px 0px 20px;
              border-left: 1px solid #222c36;
              border-bottom: 1px solid #222c36;
              background-color: #16202A;
              display: flex;
              justify-content: space-evenly;
              flex-direction: column;
              padding: 13px;
            "
          >
            <div
              style="
                color: white;
                display: flex;
                flex-direction: row;
                padding-top: 5px;
                padding-bottom: 5px;
                padding-left: 10px;
                padding-right: 10px;
                background-color: #ffffff33;
                border-radius: 5px 5px 5px 5px;
                height: 25px;
                display: flex;
                align-items: center;
              "
            >
              <Logout height={"22px".to_owned()} width={"22px".to_owned()} fill={"white".to_owned()}/>
              <div
                style="
                  color: white;
                  margin-left: 10px;
                  display: flex;
                  align-items: center;
                "
              >
                {"Logout"}
              </div>
            </div>
          </div>
      </header>
    }
}