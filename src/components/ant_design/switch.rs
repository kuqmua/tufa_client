use crate::components::ant_design::icon::Icon;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_type::SvgType;
use crate::components::rc::rc_switch::component::MouseOrKeyboardEvent;
use crate::components::rc::rc_switch::component::RcSwitch;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::NodeRef;
use yew::Properties;

// import * as React from 'react';
// import * as PropTypes from 'prop-types';
// import RcSwitch from 'rc-switch';
// import classNames from 'classnames';
// import omit from 'omit.js';
// import Wave from '../_util/wave';
// import Icon from '../icon';
// import { ConfigConsumer, ConfigConsumerProps } from '../config-provider';
// import warning from '../_util/warning';

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SwitchSize {
    Small,
    Default,
}

pub type SwitchChangeEventHandler = fn(checked: bool, event: MouseEvent) -> ();

pub type SwitchClickEventHandler = SwitchChangeEventHandler;

// export type SwitchSize = 'small' | 'default';
// export type SwitchChangeEventHandler = (checked: boolean, event: MouseEvent) => void;
// export type SwitchClickEventHandler = SwitchChangeEventHandler;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct SwitchProps {
    pub prefix_cls: Option<String>,
    pub size: Option<SwitchSize>,
    pub class: Option<String>,
    pub checked: Option<()>,
    pub default_checked: Option<()>,
    pub on_change: Option<Callback<(bool, MouseOrKeyboardEvent)>>, //todo
    pub on_click: Option<Callback<(bool, MouseEvent)>>,
    pub checked_children: Option<Html>,   //todo
    pub unchecked_children: Option<Html>, //todo
    pub disabled: Option<()>,
    pub loading: Option<()>,
    pub auto_focus: Option<()>,
    pub style: Option<String>,
    pub title: Option<String>,
}

// export interface SwitchProps {
//   prefixCls?: string;
//   size?: SwitchSize;
//   className?: string;
//   checked?: boolean;
//   defaultChecked?: boolean;
//   onChange?: SwitchChangeEventHandler;
//   onClick?: SwitchClickEventHandler;
//   checkedChildren?: React.ReactNode;
//   unCheckedChildren?: React.ReactNode;
//   disabled?: boolean;
//   loading?: boolean;
//   autoFocus?: boolean;
//   style?: React.CSSProperties;
//   title?: string;
// }

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    //
    let _ant_switch = true;

    //   static propTypes = {
    //     prefixCls: PropTypes.string,
    //     // HACK: https://github.com/ant-design/ant-design/issues/5368
    //     // size=default and size=large are the same
    //     size: PropTypes.oneOf(['small', 'default', 'large']) as PropTypes.Requireable<
    //       SwitchProps['size']
    //     >,
    //     className: PropTypes.string,
    //   };

    //   private rcSwitch: typeof RcSwitch;

    //   constructor(props: SwitchProps) {
    //     super(props);

    //     warning(
    //       'checked' in props || !('value' in props),
    //       'Switch',
    //       '`value` is not validate prop, do you mean `checked`?',
    //     );
    //   }
    // let mut rc_switch = html! {};
    // let save_switch = |node: RcSwitch| {
    //     rc_switch = html! {<RcSwitch/>};
    // };
    //   saveSwitch = (node: typeof RcSwitch) => {
    //     this.rcSwitch = node;
    //   };

    //   focus() {
    //     this.rcSwitch.focus();
    //   }

    //   blur() {
    //     this.rcSwitch.blur();
    //   }
    let render_switch = || {
        let prefix_cls = match props.prefix_cls.clone() {
            None => String::from("switch"),
            Some(pc) => pc,
        };
        let size = match props.size.clone() {
            None => SwitchSize::Default,
            Some(s) => s,
        };
        let class = match props.class.clone() {
            None => String::from(""),
            Some(c) => c,
        };
        let classes = match (size, props.loading.clone()) {
            (SwitchSize::Small, Some(_)) => {
                format!("{} {}-small {}-loading", class, prefix_cls, prefix_cls)
            }
            (SwitchSize::Small, None) => format!("{} {}-small", class, prefix_cls),
            (SwitchSize::Default, Some(_)) => format!("{} {}-loading", class, prefix_cls),
            (SwitchSize::Default, None) => class,
        };
        let loading_icon = match props.loading.clone() {
            None => html! {},
            Some(_) => html! {
                <Icon
                  svg_type={SvgType::Loading(SvgProps {
                    height: None,
                    width: None,
                    fill: None,
                    spin: None,
                    rotate: None,
                    theme: None,
                  })}
                  additional_class={format!("{}-loading-icon", prefix_cls)}
                />
            },
        };
        let disabled_handler = match (props.disabled, props.loading) {
            (None, None) => todo!(),
            (None, Some(_)) => todo!(),
            (Some(_), None) => todo!(),
            (Some(_), Some(_)) => todo!(),
        };
        html! {
        //<Wave insertExtraNode>
            // <RcSwitch
            //   {...omit(this.props, ['loading'])}
            //   prefix_cls={prefix_cls.clone()}
            //   class={classes}
            //   disabled={disabled || loading}
            //   ref={this.saveSwitch}
            //   loadingIcon={loadingIcon}
            // />
        //</Wave>
        }
    };
    //   renderSwitch = ({ getPrefixCls }: ConfigConsumerProps) => {
    //     const { prefixCls: customizePrefixCls, size, loading, className = '', disabled } = this.props;
    //     const prefixCls = getPrefixCls('switch', customizePrefixCls);
    //     const classes = classNames(className, {
    //       [`${prefixCls}-small`]: size === 'small',
    //       [`${prefixCls}-loading`]: loading,
    //     });
    //     const loadingIcon = loading ? (
    //       <Icon type="loading" className={`${prefixCls}-loading-icon`} />
    //     ) : null;
    //     return (
    //       <Wave insertExtraNode>
    //         <RcSwitch
    //           {...omit(this.props, ['loading'])}
    //           prefixCls={prefixCls}
    //           className={classes}
    //           disabled={disabled || loading}
    //           ref={this.saveSwitch}
    //           loadingIcon={loadingIcon}
    //         />
    //       </Wave>
    //     );
    //   };

    //   render() {
    //     return <ConfigConsumer>{this.renderSwitch}</ConfigConsumer>;
    //   }

    //

    // let props = prop.clone();
    // let class = match props.class.clone() {
    //     None => String::from(""),
    //     Some(cn) => cn,
    // };
    // let prefix_cls = match props.prefix_cls.clone() {
    //     None => String::from("rc-switch"),
    //     Some(pc) => pc,
    // };
    // let disabled = props.disabled.clone().is_some();
    // let checked_children = match props.checked_children {
    //     None => html! {},
    //     Some(cc) => cc,
    // };
    // let unchecked_children = match props.unchecked_children {
    //     None => html! {},
    //     Some(uc) => uc,
    // };
    // let checked = props.checked.is_some();
    // let default_checked = props.default_checked.is_some();
    // let loading_icon = match props.loading_icon.clone() {
    //     None => html! {},
    //     Some(li) => li,
    // };
    // let tab_index = match props.tab_index {
    //     None => String::from(""),
    //     Some(ti) => ti.to_string(),
    // };
    // let style = match props.style.clone() {
    //     None => String::from(""),
    //     Some(s) => s,
    // };
    // let title = match props.title.clone() {
    //     None => String::from(""),
    //     Some(t) => t,
    // };
    // let inner_checked = use_state(|| match (checked, default_checked) {
    //     (true, true) => checked,
    //     (true, false) => checked,
    //     (false, true) => default_checked,
    //     (false, false) => false,
    // });
    // let inner_checked_first_cloned = inner_checked.clone();
    // let trigger_change = move |e: (bool, MouseOrKeyboardEvent)| {
    //     let mut merged_checked = *inner_checked_first_cloned;
    //     if !disabled {
    //         merged_checked = e.0;
    //         inner_checked_first_cloned.set(merged_checked);
    //         if let Some(on_change) = props.on_change {
    //             on_change.emit((merged_checked, e.1));
    //         };
    //     }
    //     merged_checked
    // };
    // let trigger_change_cloned = trigger_change.clone();
    // let on_internal_key_down = move |e: KeyboardEvent| {
    //     let trigger_change_cloned_cloned = trigger_change_cloned.clone();
    //     let code = e.code();
    //     if code == *"ArrowLeft" {
    //         //todo maybe few same codes
    //         trigger_change_cloned_cloned((false, MouseOrKeyboardEvent::KeyboardEvent(e.clone())));
    //     } else if code == *"ArrowRight" {
    //         //todo maybe few same codes
    //         trigger_change_cloned_cloned((true, MouseOrKeyboardEvent::KeyboardEvent(e.clone())));
    //     }
    //     if let Some(on_key_down) = props.on_key_down.clone() {
    //         on_key_down.emit(e);
    //     }
    // };
    // let inner_checked_cloned = inner_checked.clone();
    // let trigger_change_second_cloned = trigger_change;
    // let on_internal_click = move |e: MouseEvent| {
    //     let trigger_change_second_cloned_cloned = trigger_change_second_cloned.clone();
    //     let trigger_change_input = (
    //         !*inner_checked_cloned.clone(),
    //         MouseOrKeyboardEvent::MouseEvent(e.clone()),
    //     );
    //     let ret = trigger_change_second_cloned_cloned(trigger_change_input);
    //     if let Some(on_click) = props.on_click.clone() {
    //         on_click.emit((ret, e));
    //     }
    // };
    // let on_internal_click_cloned = on_internal_click;
    // let switch_class_name = match (*inner_checked, disabled) {
    //     (true, true) => format!(
    //         "{} {} {}-checked {}-disabled",
    //         prefix_cls, class, prefix_cls, prefix_cls
    //     ),
    //     (true, false) => format!("{} {} {}-checked", prefix_cls, class, prefix_cls),
    //     (false, true) => format!("{} {} {}-disabled", prefix_cls, class, prefix_cls),
    //     (false, false) => format!("{} {}", prefix_cls, class),
    // };
    // let inner_checked_third = *inner_checked;
    html! {}
}

// export default class Switch extends React.Component<SwitchProps, {}> {
//   static __ANT_SWITCH = true;

//   static propTypes = {
//     prefixCls: PropTypes.string,
//     // HACK: https://github.com/ant-design/ant-design/issues/5368
//     // size=default and size=large are the same
//     size: PropTypes.oneOf(['small', 'default', 'large']) as PropTypes.Requireable<
//       SwitchProps['size']
//     >,
//     className: PropTypes.string,
//   };

//   private rcSwitch: typeof RcSwitch;

//   constructor(props: SwitchProps) {
//     super(props);

//     warning(
//       'checked' in props || !('value' in props),
//       'Switch',
//       '`value` is not validate prop, do you mean `checked`?',
//     );
//   }

//   saveSwitch = (node: typeof RcSwitch) => {
//     this.rcSwitch = node;
//   };

//   focus() {
//     this.rcSwitch.focus();
//   }

//   blur() {
//     this.rcSwitch.blur();
//   }

//   renderSwitch = ({ getPrefixCls }: ConfigConsumerProps) => {
//     const { prefixCls: customizePrefixCls, size, loading, className = '', disabled } = this.props;
//     const prefixCls = getPrefixCls('switch', customizePrefixCls);
//     const classes = classNames(className, {
//       [`${prefixCls}-small`]: size === 'small',
//       [`${prefixCls}-loading`]: loading,
//     });
//     const loadingIcon = loading ? (
//       <Icon type="loading" className={`${prefixCls}-loading-icon`} />
//     ) : null;
//     return (
//       <Wave insertExtraNode>
//         <RcSwitch
//           {...omit(this.props, ['loading'])}
//           prefixCls={prefixCls}
//           className={classes}
//           disabled={disabled || loading}
//           ref={this.saveSwitch}
//           loadingIcon={loadingIcon}
//         />
//       </Wave>
//     );
//   };

//   render() {
//     return <ConfigConsumer>{this.renderSwitch}</ConfigConsumer>;
//   }
// }
