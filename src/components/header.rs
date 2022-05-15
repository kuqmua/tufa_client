use crate::routes::routes::Routes;
use yew::{function_component, html};
use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header
            style="
                height: 50px; 
                width: 100%; 
                background-color: yellow; 
                display: flex; 
                justify-content: space-between; 
                align-items: center;
                position: fixed;
            ">
            <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0z" fill="none"/><path d="M3 15h18v-2H3v2zm0 4h18v-2H3v2zm0-8h18V9H3v2zm0-6v2h18V5H3z"/></svg>
            <Link<Routes> to={Routes::Home}>{ "home" }</Link<Routes>>
            <div>
                <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
                {"----------"}
                <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
            </div>
        </header>
    }
}

// -webkit-text-size-adjust: 100%;
// font-family: ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,Segoe UI,Roboto,Helvetica Neue,Arial,Noto Sans,sans-serif,Apple Color Emoji,Segoe UI Emoji,Segoe UI Symbol,Noto Color Emoji;
// tab-size: 4;
// line-height: inherit;
// border: 0 solid #e5e7eb;
// box-sizing: border-box;
// --tw-skew-x: 0;
// --tw-skew-y: 0;
// --tw-scale-x: 1;
// --tw-pan-x: ;
// --tw-pan-y: ;
// --tw-pinch-zoom: ;
// --tw-scroll-snap-strictness: proximity;
// --tw-ordinal: ;
// --tw-slashed-zero: ;
// --tw-numeric-figure: ;
// --tw-numeric-spacing: ;
// --tw-numeric-fraction: ;
// --tw-ring-inset: ;
// --tw-ring-offset-width: 0px;
// --tw-ring-offset-color: #fff;
// --tw-ring-color: rgb(59 130 246/0.5);
// --tw-ring-offset-shadow: 0 0 #0000;
// --tw-ring-shadow: 0 0 #0000;
// --tw-shadow: 0 0 #0000;
// --tw-shadow-colored: 0 0 #0000;
// --tw-blur: ;
// --tw-brightness: ;
// --tw-contrast: ;
// --tw-grayscale: ;
// --tw-hue-rotate: ;
// --tw-invert: ;
// --tw-saturate: ;
// --tw-sepia: ;
// --tw-drop-shadow: ;
// --tw-backdrop-blur: ;
// --tw-backdrop-brightness: ;
// --tw-backdrop-contrast: ;
// --tw-backdrop-grayscale: ;
// --tw-backdrop-hue-rotate: ;
// --tw-backdrop-invert: ;
// --tw-backdrop-opacity: ;
// --tw-backdrop-saturate: ;
// --tw-backdrop-sepia: ;
// position: fixed;
// z-index: 50;
// display: flex;
// height: 4rem;
// width: 100%;
// -webkit-box-pack: center;
// justify-content: center;
// border-bottom-width: 1px;
// --tw-border-opacity: 1;
// border-color: rgb(55 65 81/var(--tw-border-opacity));
// --tw-bg-opacity: 1;
// background-color: rgb(32 33 49/var(--tw-bg-opacity));
// --tw-text-opacity: 1;
// color: rgb(255 255 255/var(--tw-text-opacity));
