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
