use crate::routes::routes::Routes;
use crate::routes::switch_routes::switch_routes;
use yew::{function_component, html};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // let is_drawer_open = use_state(|| false);
    // let is_drawer_open_cloned_first = is_drawer_open.clone();
    // let is_drawer_open_cloned_second = is_drawer_open;
    // let oninput = Callback::from(move |_| {
    //     is_drawer_open_cloned_first.set(!*is_drawer_open_cloned_first);
    // });
    html! {
      <BrowserRouter>
          <style>
          // body {
          //     background: linear-gradient(270deg, #4ad2af, #ff0000);
          //     background-size: 400% 400%;

          //     -webkit-animation: AnimationName 30s ease infinite;
          //     -moz-animation: AnimationName 30s ease infinite;
          //     animation: AnimationName 30s ease infinite;
          // }

          // @-webkit-keyframes AnimationName {
          //     0%{background-position:53% 0%}
          //     50%{background-position:48% 100%}
          //     100%{background-position:53% 0%}
          // }
          // @-moz-keyframes AnimationName {
          //     0%{background-position:53% 0%}
          //     50%{background-position:48% 100%}
          //     100%{background-position:53% 0%}
          // }
          // @keyframes AnimationName {
          //     0%{background-position:53% 0%}
          //     50%{background-position:48% 100%}
          //     100%{background-position:53% 0%}
          // }
          // body {
          //     background: linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab);
          //     background-size: 400% 400%;
          //     animation: gradient 15s ease infinite;
          //     height: 100vh;
          // }

          // @keyframes gradient {
          //     0% {
          //         background-position: 0% 50%;
          //     }
          //     50% {
          //         background-position: 100% 50%;
          //     }
          //     100% {
          //         background-position: 0% 50%;
          //     }
          // }
          {"
            body { 
              background-color: #16202A;
              margin: 0px;
              padding: 0px;
            }
          "}
          </style>
          <Switch<Routes> render={Switch::render(switch_routes)} />
      </BrowserRouter>
    }
}
