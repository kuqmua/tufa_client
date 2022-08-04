use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Children;
use yew::Html;
use yew::Properties;
use yew::UseStateHandle;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct SelectProps {
    pub values: Vec<String>,
    // pub customize_icon: Option<Html>,                //todo types
    // pub on_mouse_down: Option<Callback<MouseEvent>>, //React.MouseEventHandler<HTMLSpanElement>
    // pub on_click: Option<Callback<MouseEvent>>,      //React.MouseEventHandler<HTMLSpanElement>
    // pub children: Children,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    // let on_click = match props.on_click.clone() {
    //     None => Callback::from(|_: MouseEvent| {}),
    //     Some(okd) => okd,
    // };
    let is_open = use_state(|| false);
    let choosen_value: UseStateHandle<Option<String>> = use_state(|| None);
    let on_click = {
        let is_open_cloned = is_open.clone();
        Callback::<MouseEvent>::from(move |e: MouseEvent| {
            e.prevent_default();
            is_open_cloned.set(!*is_open_cloned);
        })
    };
    let wrapper_class = match *is_open.clone() {
        true => String::from("ant-select antd-select-open ant-select-focused ant-select-enabled"),
        false => String::from("ant-select ant-select-enabled"),
    };
    let choosen_text = match &*choosen_value.clone() {
        None => String::from(""),
        Some(cv) => cv.to_string(),
    };
    let some_class= String::from("ant-select ant-select-enabled ant-select-dropdown-menu  ant-select-dropdown-menu-root ant-select-dropdown-menu-vertical");
    html! {
            <form>


            <div
              class={wrapper_class.clone()} style="width: 120px;"
              onclick={on_click.clone()}
            >
              <div
                class="ant-select-selection ant-select-selection--single"
                role="combobox"
                aria-autocomplete="list"
                aria-haspopup="true"
                aria-controls="32cd24a0-d717-4827-d2ce-3abe037fd4c9"
                aria-expanded="false"
                tabindex="0"
              >
                <div class="ant-select-selection__rendered">
                  <div class="ant-select-selection-selected-value" title={choosen_text.clone()} style="display: block; opacity: 1;">
                  {choosen_text}
                  </div>
                </div>
                <span class="ant-select-arrow" unselectable="on" style="user-select: none;">
                  <i aria-label="icon: down" class="anticon anticon-down ant-select-arrow-icon">
                    <svg viewBox="64 64 896 896" focusable="false" class="" data-icon="down" width="1em" height="1em" fill="currentColor" aria-hidden="true">
                      <path d="M884 256h-75c-5.1 0-9.9 2.5-12.9 6.6L512 654.2 227.9 262.6c-3-4.1-7.8-6.6-12.9-6.6h-75c-6.5 0-10.3 7.4-6.5 12.7l352.6 486.1c12.8 17.6 39 17.6 51.7 0l352.6-486.1c3.9-5.3.1-12.7-6.4-12.7z">
                      </path>
                    </svg>
                  </i>
                </span>
              </div>
            </div>

            <label for="framework">{"Select a JS Framework"}</label>
            <select
            //   id="framework"
              class={some_class}
              style="width: 120px;"
              onclick={on_click}
            >
                <option
                  class={"ant-select-dropdown-menu-item"}
                  value="1"
                >
                  {"Angular"}
                </option>
                <option
                  class={"ant-select-dropdown-menu-item"}
                  value="2"
                >
                  {"React"}
                </option>
            </select>
            // <button id="btn">{"Get the Selected Index"}</button>
        // <script>
        //     const btn = document.querySelector('#btn');
        //     const sb = document.querySelector('#framework')
        //     btn.onclick = (event) => {
        //         event.preventDefault();
        //         // show the selected index
        //         alert(sb.selectedIndex);
        //     };
        // </script>


        <div
          class="ant-select-dropdown ant-select-dropdown--single ant-select-dropdown-placement-bottomLeft"//ant-select-dropdown-hidden
        //   style="width: 120px;"// left: 24px; top: 60px;
        >
          <div
            id="8d76b3d8-779e-4642-cfc5-d49a7f277449"
            style="overflow: auto; transform: translateZ(0px);"
          >
            <ul
              role="listbox"
              class="ant-select-dropdown-menu  ant-select-dropdown-menu-root ant-select-dropdown-menu-vertical"
              tabindex="0"
            >
              <li
                role="option"
                unselectable="on"
                class="ant-select-dropdown-menu-item"
                aria-selected="false"
                style="user-select: none;"
              >
              {"Jack"}
              </li>
              <li
                role="option"
                unselectable="on"
                class="ant-select-dropdown-menu-item ant-select-dropdown-menu-item-active ant-select-dropdown-menu-item-selected"
                aria-selected="true"
                style="user-select: none;"
              >
              {"Lucy"}
              </li>
            </ul>
          </div>
        </div>

        <div
        //   style="width: 120px;"
        >
            <select
      class="form-select"
    //   aria-label=".form-select-sm example"

    >
      <option selected={true}>{"pen this select menu"}</option>
      <option value="1">{"One"}</option>
      <option value="2">{"Two"}</option>
      <option value="3">{"Three}"}</option>
    </select>
        </div>


        </form>
        }
}
