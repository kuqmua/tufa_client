use stylist::style;
use stylist::Style;
use yew::prelude::*;
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SvgIconWrapperProps {
    pub message: String,
}

pub enum Msg {
    AddOne,
    Other,
}
pub struct SvgIconWrapper {
    value: i64,
    pub stylesheet: Style,
}

impl SvgIconWrapper {
    fn style() -> Style {
        style!(
            "
    margin: 8px;
    background-color: #19857b;
    color: #fff;
    width: 40px;
    height: 40px;
    display: flex;
    overflow: hidden;
    position: relative;
    font-size: 1.25rem;
    align-items: center;
    flex-shrink: 0;
    font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
    line-height: 1;
    user-select: none;
    border-radius: 50%;
    justify-content: center;
    "
        )
        .unwrap()
    }
}

impl Component for SvgIconWrapper {
    type Message = Msg;
    type Properties = SvgIconWrapperProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            stylesheet: Self::style(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::Other => false,
        }
    }
    //https://codepen.io/shawnc8160/pen/xxRYOWg
    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        // let link = ctx.link();
        html! {
          <div
            class={self.stylesheet.clone()}
          >
          {ctx.props().message.clone()}
            <svg
              style="
                width: 75%;
                height: 75%;
                fill: currentColor;
                width: 1em;
                height: 1em;
                display: inline-block;
                font-size: 1.5rem;
                transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
                flex-shrink: 0;
                user-select: none;
              "
              class="MuiSvgIcon-root MuiAvatar-fallback"
              focusable="false"
              viewBox="0 0 24 24"
              aria-hidden="true"
            >
              <path
                d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"
              >
              </path>
            </svg>
          </div>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
