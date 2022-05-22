use crate::components::drawer::component::Drawer;
use crate::components::header::component::Header;
use crate::routes::routes::Routes;
use crate::routes::switch_routes::switch_routes;
use yew::prelude::*;
use yew::{function_component, html, Callback};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let drawer_opened = use_state(|| false);
    let drawer_opened_cloned = drawer_opened.clone();
    let drawer_opened_cloned_second = drawer_opened;
    let oninput = Callback::from(move |_| {
        drawer_opened_cloned.set(!*drawer_opened_cloned);
    });
    let transform = if *drawer_opened_cloned_second {
        "none".to_string()
    } else {
        "translateX(-100%)".to_string()
    };
    let aside_navigation_style = format!(
        "
      position: fixed;
      z-index: 99;
      width: 350px;
      height: 100%;
      top: 0;
      bottom: 0;
      transform: {};
      display: grid;
      transition: transform 0.3s cubic-bezier(0.4, 0.0, 0.2, 1);
    ",
        transform
    );
    let aside_label_style = format!(
        "
      position: fixed;
      z-index: 98;
      width: 100%;
      height: 100%;
      top: 0;
      bottom: 0;
      transform: {};
      display: grid;
      opacity: 0.5;
  
    ",
        transform
    );
    html! {
      <BrowserRouter>
        <Header callback={oninput.clone()}/>
        <Drawer aside_navigation_style={aside_navigation_style} aside_label_style={aside_label_style} callback={oninput}/>
        <div
          style="
          width: 100%; 
          display: flex; 
          justify-content: center; 
          flex-direction: column; 
          align-items: center;
        "
        >
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
        </div>
      </BrowserRouter>
    }
}
