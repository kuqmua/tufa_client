use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Children;
use yew::Html;
use yew::Properties;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct SelectProps {
    // pub class_names: Vec<String>,
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
    html! {
        <div class="ant-select ant-select-enabled" style="width: 120px;">
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
              <div class="ant-select-selection-selected-value" title="Lucy" style="display: block; opacity: 1;">
              {"Lucy"}
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
    }
}
