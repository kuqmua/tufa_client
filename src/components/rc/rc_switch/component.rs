use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Children;
use yew::Html;
use yew::Properties;
// use yew_stdweb::events::ChangeData;
// use yewdux::prelude::Changed;

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
    pub checked_children: Children,
    pub unchecked_children: Children,
    pub on_change: Option<Callback<(bool, MouseOrKeyboardEvent)>>,
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_click: Option<Callback<(bool, MouseEvent)>>,
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

#[derive(Debug, PartialEq, Clone)]
pub enum MouseOrKeyboardEvent {
    MouseEvent(MouseEvent),
    KeyboardEvent(KeyboardEvent),
}

#[function_component(Switch)]
pub fn switch(prop: &SwitchProps) -> Html {
    let props = prop.clone();
    let class = match props.class.clone() {
        None => String::from(""),
        Some(cn) => cn,
    };
    let prefix_cls = match props.prefix_cls.clone() {
        None => String::from("rc-switch"),
        Some(pc) => pc,
    };
    let disabled = props.disabled.clone().is_some();
    let on_key_down = match props.on_key_down.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(okd) => okd,
    };
    let on_click = match props.on_click.clone() {
        None => Callback::from(|_: (bool, MouseEvent)| {}),
        Some(okd) => okd,
    };
    let tab_index = match props.tab_index {
        None => String::from(""),
        Some(ti) => ti.to_string(),
    };
    let checked = props.checked.is_some();
    let default_checked = props.default_checked.is_some();
    let loading_icon = match props.loading_icon.clone() {
        None => html! {},
        Some(li) => li,
    };
    let style = match props.style.clone() {
        None => String::from(""),
        Some(s) => s,
    };
    let title = match props.title.clone() {
        None => String::from(""),
        Some(t) => t,
    };
    let inner_checked = use_state(|| match (checked, default_checked) {
        (true, true) => checked,
        (true, false) => checked,
        (false, true) => default_checked,
        (false, false) => false,
    });
    let inner_checked_first_cloned = inner_checked.clone();
    let trigger_change = move |e: (bool, MouseOrKeyboardEvent)| {
        //todo KeyBoardevent
        let mut merged_checked = *inner_checked_first_cloned;
        if !disabled {
            merged_checked = e.0;
            inner_checked_first_cloned.set(merged_checked);
            if let Some(on_change) = props.on_change {
                on_change.emit((merged_checked, e.1));
            };
        }
        merged_checked
    };
    let trigger_change_cloned = trigger_change.clone();
    let on_internal_key_down = move |e: KeyboardEvent| {
        let trigger_change_cloned_cloned = trigger_change_cloned.clone();
        let code = e.code();
        // //todo
        if code == *"LEFT" {
            trigger_change_cloned_cloned((false, MouseOrKeyboardEvent::KeyboardEvent(e)));
        } else if code == *"RIGHT" {
            trigger_change_cloned_cloned((true, MouseOrKeyboardEvent::KeyboardEvent(e)));
        }
        // //   onKeyDown?.(e);
    };

    //     function onInternalKeyDown(e: React.KeyboardEvent<HTMLButtonElement>) {
    //       if (e.which === KeyCode.LEFT) {
    //         triggerChange(false, e);
    //       } else if (e.which === KeyCode.RIGHT) {
    //         triggerChange(true, e);
    //       }
    //       onKeyDown?.(e);
    //     }
    let inner_checked_cloned = inner_checked.clone();
    let trigger_change_second_cloned = trigger_change.clone();
    let on_internal_click = move |e: MouseEvent| {
        let trigger_change_second_cloned_cloned = trigger_change_second_cloned.clone();
        let on_click_cloned = on_click.clone();
        let trigger_change_input = (
            !*inner_checked_cloned.clone(),
            MouseOrKeyboardEvent::MouseEvent(e.clone()),
        );
        let ret = trigger_change_second_cloned_cloned(trigger_change_input);
        on_click_cloned.emit((ret, e));
    };
    let on_internal_click_cloned = on_internal_click;

    //     function onInternalClick(e: React.MouseEvent<HTMLButtonElement>) {
    //       const ret = triggerChange(!innerChecked, e);
    //       // [Legacy] trigger onClick with value
    //       onClick?.(ret, e);
    //     }

    let switch_class_name = match (*inner_checked, disabled) {
        (true, true) => format!(
            "{} {} {}-checked {}-disabled",
            prefix_cls, class, prefix_cls, prefix_cls
        ),
        (true, false) => format!("{} {} {}-checked", prefix_cls, class, prefix_cls),
        (false, true) => format!("{} {} {}-disabled", prefix_cls, class, prefix_cls),
        (false, false) => format!("{} {}", prefix_cls, class),
    };
    let inner_checked_third = *inner_checked;
    html! {
      <button
        // {...restProps}
        type="button"
        role="switch"
        aria-checked={inner_checked_third.clone().to_string()}
        disabled={disabled}
        class={switch_class_name.clone()}
        // ref={ref}
        onkeydown={on_internal_key_down}
        onclick={on_internal_click_cloned}
      >
        {loading_icon.clone()}
        <span class={format!("{}-inner", prefix_cls.clone())}>
        if *inner_checked {
            {props.checked_children.clone()}
        }
        else {
            {props.unchecked_children.clone()}
        }
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
