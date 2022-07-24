use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Html;
use yew::Properties;

// import * as React from 'react';
// import classNames from 'classnames';
// import useMergedState from 'rc-util/lib/hooks/useMergedState';
// import KeyCode from 'rc-util/lib/KeyCode';

// export type SwitchChangeEventHandler = (
//   checked: boolean,
//   event: React.MouseEvent<HTMLButtonElement> | React.KeyboardEvent<HTMLButtonElement>,
// ) => void;
// export type SwitchClickEventHandler = SwitchChangeEventHandler;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct SwitchProps {
    pub class: Option<String>,
    pub prefix_cls: Option<String>,
    pub disabled: Option<()>,
    pub checked_children: Option<Html>,   //Children
    pub unchecked_children: Option<Html>, //Children
    //   pub on_change?: SwitchChangeEventHandler; //todo
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_click: Option<Callback<MouseEvent>>, //todo
    pub tab_index: Option<i32>,
    pub checked: Option<()>,
    pub default_checked: Option<()>,
    pub loading_icon: Option<Html>, //todo
    pub style: Option<String>,
    pub title: Option<String>,
}

// interface SwitchProps
//   extends Omit<React.HTMLAttributes<HTMLButtonElement>, 'onChange' | 'onClick'> {
//   className?: string;
//   prefixCls?: string;
//   disabled?: boolean;
//   checkedChildren?: React.ReactNode;
//   unCheckedChildren?: React.ReactNode;
//   onChange?: SwitchChangeEventHandler;
//   onKeyDown?: React.KeyboardEventHandler<HTMLButtonElement>;
//   onClick?: SwitchClickEventHandler;
//   tabIndex?: number;
//   checked?: boolean;
//   defaultChecked?: boolean;
//   loadingIcon?: React.ReactNode;
//   style?: React.CSSProperties;
//   title?: string;
// }

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let class = match props.class.clone() {
        None => String::from(""),
        Some(cn) => cn,
    };
    let prefix_cls = match props.prefix_cls.clone() {
        None => String::from("rc-switch"),
        Some(pc) => pc,
    };
    let disabled = props.disabled.clone().is_some();
    // pub checked_children: Option<Html>,   //Children
    // pub unchecked_children: Option<Html>, //Children
    // //   pub on_change?: SwitchChangeEventHandler; //todo
    let on_key_down = match props.on_key_down.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(okd) => okd,
    };
    let on_click = match props.on_click.clone() {
        None => Callback::from(|_: MouseEvent| {}),
        Some(okd) => okd,
    };
    let tab_index = match props.tab_index {
        None => String::from(""),
        Some(ti) => ti.to_string(),
    };
    let checked = props.checked.is_some();
    let default_checked = props.default_checked.is_some();
    // pub loading_icon: Option<Html>, //todo
    let style = match props.style.clone() {
        None => String::from(""),
        Some(s) => s,
    };
    let title = match props.title.clone() {
        None => String::from(""),
        Some(t) => t,
    };
    //     const [innerChecked, setInnerChecked] = useMergedState<boolean>(false, {
    //       value: checked,
    //       defaultValue: defaultChecked,
    //     });
    // let trigger_change: |(MouseEvent, bool)|{} = |(event, new_checked)| {
    // //   let mergedChecked = innerChecked;
    // //   if (!disabled) {
    // //     mergedChecked = newChecked;
    // //     setInnerChecked(mergedChecked);
    // //     onChange?.(mergedChecked, event);
    // //   }
    // };
    //     function triggerChange(
    //       newChecked: boolean,
    //       event: React.MouseEvent<HTMLButtonElement> | React.KeyboardEvent<HTMLButtonElement>,
    //     ) {
    //       let mergedChecked = innerChecked;

    //       if (!disabled) {
    //         mergedChecked = newChecked;
    //         setInnerChecked(mergedChecked);
    //         onChange?.(mergedChecked, event);
    //       }

    //       return mergedChecked;
    //     }

    //     function onInternalKeyDown(e: React.KeyboardEvent<HTMLButtonElement>) {
    //       if (e.which === KeyCode.LEFT) {
    //         triggerChange(false, e);
    //       } else if (e.which === KeyCode.RIGHT) {
    //         triggerChange(true, e);
    //       }
    //       onKeyDown?.(e);
    //     }

    //     function onInternalClick(e: React.MouseEvent<HTMLButtonElement>) {
    //       const ret = triggerChange(!innerChecked, e);
    //       // [Legacy] trigger onClick with value
    //       onClick?.(ret, e);
    //     }

    // let switch_class_name = match (checked, disabled) {
    //     (true, true) => format!(
    //         "{} {} {}-checked {}-disabled",
    //         prefix_cls, class_name, prefix_cls, prefix_cls
    //     ),
    //     (true, false) => format!("{} {} {}-checked", prefix_cls, class_name, prefix_cls),
    //     (false, true) => format!("{} {} {}-disabled", prefix_cls, class_name, prefix_cls),
    //     (false, false) => format!("{} {}", prefix_cls, class_name),
    // };

    //     const switchClassName = classNames(prefixCls, className, {
    //       [`${prefixCls}-checked`]: innerChecked,
    //       [`${prefixCls}-disabled`]: disabled,
    //     });
    html! {
      <button
        // {...restProps}
        type="button"
        role="switch"
        // aria-checked={inner_checked}
        disabled={disabled}
        // class={switch_class_name}
        // ref={ref}
        // on-key-down={on_internal_key_down}
        // onclick={on_internal_click}
      >
        // {loading_icon}
        <span class={format!("{}-inner", prefix_cls)}>
        //   {innerChecked ? checkedChildren : unCheckedChildren}
        </span>
      </button>
    }
}

// const Switch = React.forwardRef<HTMLButtonElement, SwitchProps>(
//   (
//     {
//       prefixCls = 'rc-switch',
//       className,
//       checked,
//       defaultChecked,
//       disabled,
//       loadingIcon,
//       checkedChildren,
//       unCheckedChildren,
//       onClick,
//       onChange,
//       onKeyDown,
//       ...restProps
//     },
//     ref,
//   ) => {
//     const [innerChecked, setInnerChecked] = useMergedState<boolean>(false, {
//       value: checked,
//       defaultValue: defaultChecked,
//     });

//     function triggerChange(
//       newChecked: boolean,
//       event: React.MouseEvent<HTMLButtonElement> | React.KeyboardEvent<HTMLButtonElement>,
//     ) {
//       let mergedChecked = innerChecked;

//       if (!disabled) {
//         mergedChecked = newChecked;
//         setInnerChecked(mergedChecked);
//         onChange?.(mergedChecked, event);
//       }

//       return mergedChecked;
//     }

//     function onInternalKeyDown(e: React.KeyboardEvent<HTMLButtonElement>) {
//       if (e.which === KeyCode.LEFT) {
//         triggerChange(false, e);
//       } else if (e.which === KeyCode.RIGHT) {
//         triggerChange(true, e);
//       }
//       onKeyDown?.(e);
//     }

//     function onInternalClick(e: React.MouseEvent<HTMLButtonElement>) {
//       const ret = triggerChange(!innerChecked, e);
//       // [Legacy] trigger onClick with value
//       onClick?.(ret, e);
//     }

//     const switchClassName = classNames(prefixCls, className, {
//       [`${prefixCls}-checked`]: innerChecked,
//       [`${prefixCls}-disabled`]: disabled,
//     });

//     return (
//       <button
//         {...restProps}
//         type="button"
//         role="switch"
//         aria-checked={innerChecked}
//         disabled={disabled}
//         className={switchClassName}
//         ref={ref}
//         onKeyDown={onInternalKeyDown}
//         onClick={onInternalClick}
//       >
//         {loadingIcon}
//         <span className={`${prefixCls}-inner`}>
//           {innerChecked ? checkedChildren : unCheckedChildren}
//         </span>
//       </button>
//     );
//   },
// );
// Switch.displayName = 'Switch';
