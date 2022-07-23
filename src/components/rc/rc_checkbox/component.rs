// // eslint-disable-next-line import/no-extraneous-dependencies
// import React, { Component } from 'react';
// import classNames from 'classnames';

use crate::components::rc::rc_checkbox::types::InputType;
use crate::components::rc::rc_checkbox::types::RcCheckBoxProps;
use web_sys::Event;
use web_sys::FocusEvent;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;

#[function_component(RcCheckBox)]
pub fn rc_checkbox(props: &RcCheckBoxProps) -> Html {
    let prefix_cls = match props.prefix_cls.clone() {
        None => String::from("rc-checkbox"),
        Some(pc) => pc,
    };
    let class_name = match props.class_name.clone() {
        None => String::from(""),
        Some(cn) => cn,
    };
    let style = match props.style.clone() {
        None => String::from(""),
        Some(s) => s,
    };
    let type_handle = match props.type_handle.clone() {
        None => InputType::Checkbox,
        Some(t) => t,
    };
    let title = match props.title.clone() {
        None => String::from(""),
        Some(t) => t,
    };
    let default_checked = match props.default_checked.clone() {
        None => false,
        Some(_) => true,
    };
    let checked = match props.checked.clone() {
        None => default_checked,
        Some(_) => true,
    };
    //
    let name = match props.name.clone() {
        None => String::from(""),
        Some(n) => n,
    };
    let id = match props.id.clone() {
        None => String::from(""),
        Some(i) => i,
    };
    let required = props.required.clone().unwrap_or(false);
    let read_only = match props.read_only.clone() {
        None => false,
        Some(r) => r,
    };
    let disabled = match props.disabled.clone() {
        None => false,
        Some(r) => r,
    };
    let tab_index = match props.tab_index.clone() {
        None => String::from(""),
        Some(r) => r.to_string(),
    };
    //
    let checked_state = use_state(|| checked);
    let on_focus = match props.on_focus.clone() {
        None => Callback::from(|_: FocusEvent| {}),
        Some(of) => of,
    };
    let on_blur = match props.on_blur.clone() {
        None => Callback::from(|_: FocusEvent| {}),
        Some(ob) => ob,
    };
    let on_change = match props.on_change.clone() {
        None => Callback::from(|_: Event| {}),
        Some(oc) => oc,
    };
    let on_click = match props.on_click.clone() {
        None => Callback::from(|_: MouseEvent| {}),
        Some(okd) => okd,
    };
    let on_key_down = match props.on_key_down.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(okd) => okd,
    };
    let on_key_press = match props.on_key_press.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(okp) => okp,
    };
    let on_key_up = match props.on_key_up.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(oku) => oku,
    };
    let value = match props.value.clone() {
        None => String::from(""),
        Some(v) => v,
    };

    // let focus = || {
    //   this.input.focus();
    // };

    // let blur = || {
    //   this.input.blur();
    // };

    let checked_state_cloned = checked_state;
    let disabled_cloned = props.disabled;
    let on_change_cloned: Option<Callback<Event>> = props.on_change.clone();
    let checked_cloned = props.checked;
    let handle_change = move |_e: Event| {
        if disabled_cloned.is_some() {
            return;
        }
        if checked_cloned.is_none() {
            // checked_state_cloned.set(e.target.checked);
        }
        if let Some(on_change_handle) = on_change_cloned.clone() {
            //   onChange({
            //     target: {
            //       ...this.props,
            //       checked: e.target.checked,
            //     },
            //     stopPropagation() {
            //       e.stopPropagation();
            //     },
            //     preventDefault() {
            //       e.preventDefault();
            //     },
            //     nativeEvent: e.nativeEvent,
            //   });
        }
    };
    // let save_input = |node| {
    //     this.input = node;
    // };

    //     const {
    //       prefixCls,
    //       className,
    //       style,
    //       name,
    //       id,
    //       type,
    //       title,
    //       disabled,
    //       readOnly,
    //       tabIndex,
    //       onClick,
    //       onFocus,
    //       onBlur,
    //       onKeyDown,
    //       onKeyPress,
    //       onKeyUp,
    //       autoFocus,
    //       value,
    //       required,
    //       ...others
    //     } = this.props;

    //     const globalProps = Object.keys(others).reduce((prev, key) => {
    //       if (key.substr(0, 5) === 'aria-' || key.substr(0, 5) === 'data-' || key === 'role') {
    //         // eslint-disable-next-line no-param-reassign
    //         prev[key] = others[key];
    //       }
    //       return prev;
    //     }, {});

    //     const { checked } = this.state;
    let class_string = format!(
        "{} {} {}-checked {}-disabled",
        prefix_cls, class_name, prefix_cls, prefix_cls
    );
    //     const classString = classNames(prefixCls, className, {
    //       [`${prefixCls}-checked`]: checked,
    //       [`${prefixCls}-disabled`]: disabled,
    //     });
    html! {
      <span class={class_string} style={style}>
        <input
          name={name}
          id={id}
          type={format!("{}", type_handle)}
          title={title}
          required={required}
          readonly={read_only}
          disabled={disabled}
          tabIndex={tab_index}
          className={format!("{}-input", prefix_cls)}
          checked={checked}
          onclick={on_click}
          onfocus={on_focus}
          onblur={on_blur}
          onkeyup={on_key_up}
          onkeydown={on_key_down}
          onkeypress={on_key_press}
          onchange={handle_change.clone()}
        //   autofocus={auto_focus}//todo
        //   ref={this.save_input}
        //   value={value.clone}//todo
        //   {...globalProps}
        />
        <span class={format!("{}-inner", prefix_cls)} />
      </span>
    }
}
// class Checkbox extends Component {
//   static defaultProps = {
//     prefixCls: 'rc-checkbox',
//     className: '',
//     style: {},
//     type: 'checkbox',
//     title: '',
//     defaultChecked: false,
//     onFocus() {},
//     onBlur() {},
//     onChange() {},
//     onKeyDown() {},
//     onKeyPress() {},
//     onKeyUp() {},
//   };

//   constructor(props) {
//     super(props);

//     const checked = 'checked' in props ? props.checked : props.defaultChecked;

//     this.state = {
//       checked,
//     };
//   }

//   static getDerivedStateFromProps(props, state) {
//     if ('checked' in props) {
//       return {
//         ...state,
//         checked: props.checked,
//       };
//     }
//     return null;
//   }

//   focus() {
//     this.input.focus();
//   }

//   blur() {
//     this.input.blur();
//   }

//   handleChange = e => {
//     const { disabled, onChange } = this.props;
//     if (disabled) {
//       return;
//     }
//     if (!('checked' in this.props)) {
//       this.setState({
//         checked: e.target.checked,
//       });
//     }
//     if (onChange) {
//       onChange({
//         target: {
//           ...this.props,
//           checked: e.target.checked,
//         },
//         stopPropagation() {
//           e.stopPropagation();
//         },
//         preventDefault() {
//           e.preventDefault();
//         },
//         nativeEvent: e.nativeEvent,
//       });
//     }
//   };

//   saveInput = node => {
//     this.input = node;
//   };

//   render() {
//     const {
//       prefixCls,
//       className,
//       style,
//       name,
//       id,
//       type,
//       title,
//       disabled,
//       readOnly,
//       tabIndex,
//       onClick,
//       onFocus,
//       onBlur,
//       onKeyDown,
//       onKeyPress,
//       onKeyUp,
//       autoFocus,
//       value,
//       required,
//       ...others
//     } = this.props;

//     const globalProps = Object.keys(others).reduce((prev, key) => {
//       if (key.substr(0, 5) === 'aria-' || key.substr(0, 5) === 'data-' || key === 'role') {
//         // eslint-disable-next-line no-param-reassign
//         prev[key] = others[key];
//       }
//       return prev;
//     }, {});

//     const { checked } = this.state;
//     const classString = classNames(prefixCls, className, {
//       [`${prefixCls}-checked`]: checked,
//       [`${prefixCls}-disabled`]: disabled,
//     });

//     return (
//       <span className={classString} style={style}>
//         <input
//           name={name}
//           id={id}
//           type={type}
//           title={title}
//           required={required}
//           readOnly={readOnly}
//           disabled={disabled}
//           tabIndex={tabIndex}
//           className={`${prefixCls}-input`}
//           checked={!!checked}
//           onClick={onClick}
//           onFocus={onFocus}
//           onBlur={onBlur}
//           onKeyUp={onKeyUp}
//           onKeyDown={onKeyDown}
//           onKeyPress={onKeyPress}
//           onChange={this.handleChange}
//           autoFocus={autoFocus}
//           ref={this.saveInput}
//           value={value}
//           {...globalProps}
//         />
//         <span className={`${prefixCls}-inner`} />
//       </span>
//     );
//   }
// }

// export default Checkbox;
