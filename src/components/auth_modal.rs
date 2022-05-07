use crate::components::input_button::InputButton;
use crate::components::input_form::InputForm;
use crate::components::svg_icon_wrapper::SvgIconWrapper;
use crate::routes::routes::Routes;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AuthModal)]
pub fn auth_modal() -> Html {
    let history_option = use_history();
    let onclick: Callback<MouseEvent>;
    match history_option {
        Some(history) => onclick = Callback::once(move |_| history.push(Routes::Secure)),
        None => onclick = Callback::once(move |_| log!("Update1")),
    }
    html! {
     <div
       id="root"
       style="
      display: block;
      color: rgba(0, 0, 0, 0.87);
      margin: 0;
      font-size: 0.875rem;
      font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
      font-weight: 400;
      line-height: 1.43;
      letter-spacing: 0.01071em;
      background-color: #fff;
      -webkit-font-smoothing: antialiased;
      box-sizing: inherit;
  "
     >
       <div
         class="MuiContainer-root MuiContainer-maxWidthXs"
         style="
        max-width: 444px;
        width: 100%;
        display: block;
        box-sizing: border-box;
        margin-left: auto;
        margin-right: auto;
        padding-left: 16px;
        padding-right: 16px;
      "
       >
         <div
           class="makeStyles-paper-1"
           style="
          display: flex;
          margin-top: 64px;
          align-items: center;
          flex-direction: column;
        "
         >
           <SvgIconWrapper/>
           <h1
             class="MuiTypography-root MuiTypography-h5"
             style="
            font-size: 1.5rem;
            font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
            font-weight: 400;
            line-height: 1.334;
            letter-spacing: 0em;
            margin: 0;
            box-sizing: inherit;
            display: block;
            font-size: 2em;
            margin-block-start: 0.67em;
            margin-block-end: 0.67em;
            margin-inline-start: 0px;
            margin-inline-end: 0px;
            font-weight: bold;
          "
           >
             {"Sign up"}
           </h1>
           <form
             class="makeStyles-form-3"
             novalidate=true
             style="
            width: 100%;
            margin-top: 24px;
            box-sizing: inherit;
            display: block;
          "
           >
             <div
               class="MuiGrid-root MuiGrid-container MuiGrid-spacing-xs-2"
               style="
              width: calc(100% + 16px);
              margin: -8px;
              display: flex;
              flex-wrap: wrap;
              box-sizing: border-box;
            "
             >
               <InputForm message_handle={"Login".to_owned()}/>
               <InputForm message_handle={"Password".to_owned()}/>
               <div
                 class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12"
               >
               </div>
             </div>
             <InputButton />
             <div
               class="MuiGrid-root MuiGrid-container MuiGrid-justify-content-xs-flex-end"
               style="
              justify-content: flex-end;
              width: 100%;
              display: flex;
              flex-wrap: wrap;
              box-sizing: border-box;
            "
             >
               <div
                 class="MuiGrid-root MuiGrid-item"
                 style="
                margin: 0;
                box-sizing: border-box;
                display: block;
              "
               >
                 <a
                   class="MuiTypography-root MuiLink-root MuiLink-underlineHover MuiTypography-body2 MuiTypography-colorPrimary"
                   href=""
                   style="
                  text-decoration: none;
                  color: #556cd6;
                  font-size: 0.875rem;
                  font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                  font-weight: 400;
                  line-height: 1.43;
                  letter-spacing: 0.01071em;
                  margin: 0;
                  box-sizing: inherit;
                  cursor: pointer;
                "
                 >
                   {"Already have an account? Sign in"}
                 </a>
               </div>
             </div>
           </form>
         </div>
       </div>
       <button {onclick}>{ "Go to Secure" }</button>
     </div>
    }
}
