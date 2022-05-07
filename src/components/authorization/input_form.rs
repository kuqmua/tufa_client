use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputFormProps {
    pub message_handle: String,
}
pub enum InputFormMessage {
    ActionOne,
}
pub struct InputForm {}

impl Component for InputForm {
    type Message = InputFormMessage;
    type Properties = InputFormProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div
            style="
              -webkit-font-smoothing: antialiased;
              color: rgba(0, 0, 0, 0.87);
              font-size: 0.875rem;
              font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
              font-weight: 400;
              line-height: 1.43;
              letter-spacing: 0.01071em;
              margin: 0;
              box-sizing: border-box;
              flex-grow: 0;
              max-width: 100%;
              flex-basis: 100%;
              padding: 8px;
            "
          >
            <div
              style="
                -webkit-font-smoothing: antialiased;
                color: rgba(0, 0, 0, 0.87);
                font-size: 0.875rem;
                font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                font-weight: 400;
                line-height: 1.43;
                letter-spacing: 0.01071em;
                box-sizing: inherit;
                border: 0;
                margin: 0;
                display: inline-flex;
                padding: 0;
                position: relative;
                min-width: 0;
                flex-direction: column;
                vertical-align: top;
                width: 100%;
              "
            >
              <label
                data-shrink="false"
                for="email"
                id="email-label"
                style="
                  -webkit-font-smoothing: antialiased;
                  box-sizing: inherit;
                  color: rgba(0, 0, 0, 0.54);
                  padding: 0;
                  font-size: 1rem;
                  font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                  font-weight: 400;
                  line-height: 1;
                  letter-spacing: 0.00938em;
                  display: block;
                  transform-origin: top left;
                  top: 0;
                  left: 0;
                  position: absolute;
                  transition: color 200ms cubic-bezier(0.0, 0, 0.2, 1) 0ms,transform 200ms cubic-bezier(0.0, 0, 0.2, 1) 0ms;
                  z-index: 1;
                  transform: translate(14px, 20px) scale(1);
                  pointer-events: none;
                "
              >
                {ctx.props().message_handle.clone()}
              </label>
              <div
                style="
                  -webkit-font-smoothing: antialiased;
                  color: rgba(0, 0, 0, 0.87);
                  cursor: text;
                  display: inline-flex;
                  font-size: 1rem;
                  box-sizing: border-box;
                  align-items: center;
                  font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                  font-weight: 400;
                  line-height: 1.1876em;
                  letter-spacing: 0.00938em;
                  width: 100%;
                  position: relative;
                  border-radius: 4px;
                "
              >
                <input
                  aria-invalid="false"
                  autocomplete="email"
                  id="email"
                  name="email"
                  type="text"
                  value=""
                  style="
                    -webkit-font-smoothing: antialiased;
                    font: inherit;
                    color: currentColor;
                    width: 100%;
                    border: 0;
                    height: 1.1876em;
                    margin: 0;
                    display: block;
                    min-width: 0;
                    background: none;
                    box-sizing: content-box;
                    animation-name: mui-auto-fill-cancel;
                    letter-spacing: inherit;
                    animation-duration: 10ms;
                    -webkit-tap-highlight-color: transparent;
                    padding: 18.5px 14px;
                    box-shadow: none;
                  "
                />
                <fieldset
                  aria-hidden="true"
                  style="
                    -webkit-font-smoothing: antialiased;
                    color: rgba(0, 0, 0, 0.87);
                    cursor: text;
                    font-size: 1rem;
                    font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                    font-weight: 400;
                    line-height: 1.1876em;
                    letter-spacing: 0.00938em;
                    box-sizing: inherit;
                    top: -5px;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    margin: 0;
                    padding: 0 8px;
                    overflow: hidden;
                    position: absolute;
                    border-style: solid;
                    border-width: 1px;
                    border-radius: inherit;
                    pointer-events: none;
                    border-color: rgba(0, 0, 0, 0.23);
                  "
                >
                  <legend
                    style="
                      -webkit-font-smoothing: antialiased;
                      color: rgba(0, 0, 0, 0.87);
                      cursor: text;
                      font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                      font-weight: 400;
                      line-height: 1.1876em;
                      letter-spacing: 0.00938em;
                      pointer-events: none;
                      box-sizing: inherit;
                      width: auto;
                      height: 11px;
                      display: block;
                      padding: 0;
                      font-size: 0.75em;
                      max-width: 0.01px;
                      text-align: left;
                      transition: max-width 50ms cubic-bezier(0.0, 0, 0.2, 1) 0ms;
                      visibility: hidden;
                    "
                  >
                    <span
                      style="
                        -webkit-font-smoothing: antialiased;
                        color: rgba(0, 0, 0, 0.87);
                        cursor: text;
                        font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                        font-weight: 400;
                        line-height: 1.1876em;
                        letter-spacing: 0.00938em;
                        pointer-events: none;
                        font-size: 0.75em;
                        text-align: left;
                        visibility: hidden;
                        box-sizing: inherit;
                        display: inline-block;
                        padding-left: 5px;
                        padding-right: 5px;
                      "
                    >
                      {"Email Address&nbsp;*"}
                    </span>
                  </legend>
                </fieldset>
              </div>
            </div>
          </div>
        }
    }
}
