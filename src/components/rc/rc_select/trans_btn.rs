use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Children;
use yew::Html;
use yew::Properties;

// import * as React from 'react';
// import classNames from 'classnames';
// import type { RenderNode } from './BaseSelect';

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct TransBtnProps {
    pub class_name: String,
    pub customize_icon: Option<Html>,
    //   pub customize_icon_props: Option<any>,
    pub on_mouse_down: Option<Callback<MouseEvent>>, //React.MouseEventHandler<HTMLSpanElement>
    pub on_click: Option<Callback<MouseEvent>>,      //React.MouseEventHandler<HTMLSpanElement>
    pub children: Children,
}

// export interface TransBtnProps {
//   className: string;
//   customizeIcon: RenderNode;
//   customizeIconProps?: any;
//   onMouseDown?: React.MouseEventHandler<HTMLSpanElement>;
//   onClick?: React.MouseEventHandler<HTMLSpanElement>;
//   children?: React.ReactNode;
// }

#[function_component(TransBtn)]
pub fn trans_btn(props: &TransBtnProps) -> Html {
    let on_click = match props.on_click.clone() {
        None => Callback::from(|_: MouseEvent| {}),
        Some(okd) => okd,
    };
    let on_mouse_down = match props.on_mouse_down.clone() {
        None => Callback::from(|e: MouseEvent| {
            e.prevent_default();
        }),
        Some(okd) => Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            okd.emit(e);
        }),
    };
    let content = match props.customize_icon.clone() {
        None => {
            // let b = classNames(className.split(/\s+/).map((cls) => `${cls}-icon`));
            html! {
              <span class={String::from("")}>
                {props.children.clone()}
              </span>
            }
        }
        Some(ci) => ci,
    };
    html! {
      <span
        class={props.class_name.clone()}
        onmousedown={on_mouse_down.clone()}
        style={"user-select: none; -webkit-user-select: none".to_string()}
        unselectable="on"
        onclick={on_click}
        aria-hidden={"true"}
      >
        {content}
      </span>
    }
}

// const TransBtn: React.FC<TransBtnProps> = ({
//   className,
//   customizeIcon,
//   customizeIconProps,
//   onMouseDown,
//   onClick,
//   children,
// }) => {
//   let icon: React.ReactNode;

//   if (typeof customizeIcon === 'function') {
//     icon = customizeIcon(customizeIconProps);
//   } else {
//     icon = customizeIcon;
//   }

//   return (
//     <span
//       className={className}
//       onMouseDown={(event) => {
//         event.preventDefault();
//         if (onMouseDown) {
//           onMouseDown(event);
//         }
//       }}
//       style={{
//         userSelect: 'none',
//         WebkitUserSelect: 'none',
//       }}
//       unselectable="on"
//       onClick={onClick}
//       aria-hidden
//     >
//       {icon !== undefined ? (
//         icon
//       ) : (
//         <span className={classNames(className.split(/\s+/).map((cls) => `${cls}-icon`))}>
//           {children}
//         </span>
//       )}
//     </span>
//   );
// };

// export default TransBtn;
