use std::collections::HashMap;
use crate::helpers::pseudo_css_wrapper::PseudoCssWrapper;

// import * as React from 'react';
// import { polyfill } from 'react-lifecycles-compat';
// import RcTooltip from 'rc-tooltip';
// import classNames from 'classnames';
// import getPlacements, { AdjustOverflow, PlacementsConfig } from './placements';
use crate::components::ant_design::tooltip::placements::get_placements;
use crate::components::ant_design::tooltip::placements::AdjustOverflow;
use crate::components::ant_design::tooltip::placements::AdjustOverflowOrBool;
use crate::components::ant_design::tooltip::placements::PlacementsConfig;
// import { ConfigConsumer, ConfigConsumerProps } from '../config-provider';

// export { AdjustOverflow, PlacementsConfig };

use crate::components::ant_design::helpers::offset::Offset;
use yew::{function_component, html, Callback, Children, Html, Properties};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TooltipPlacement {
    Top,
    Left,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

// export type TooltipPlacement =
//   | 'top'
//   | 'left'
//   | 'right'
//   | 'bottom'
//   | 'topLeft'
//   | 'topRight'
//   | 'bottomLeft'
//   | 'bottomRight'
//   | 'leftTop'
//   | 'leftBottom'
//   | 'rightTop'
//   | 'rightBottom';

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Overflow {
    pub adjust_x: bool,
    pub adjust_y: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TooltipAlignConfig {
    pub points: Option<(String, String)>,
    pub offset: Option<Offset>,
    pub target_offset: Option<Offset>,
    pub overflow: Option<Overflow>,
    pub use_css_right: Option<()>,
    pub use_css_bottom: Option<()>,
    pub use_css_transform: Option<()>,
}

// // https://github.com/react-component/tooltip
// // https://github.com/yiminghe/dom-align
// export interface TooltipAlignConfig {
//   points?: [string, string];
//   offset?: [number | string, number | string];
//   targetOffset?: [number | string, number | string];
//   overflow?: { adjustX: boolean; adjustY: boolean };
//   useCssRight?: boolean;
//   useCssBottom?: boolean;
//   useCssTransform?: boolean;
// }

#[derive(Debug, PartialEq, Clone)]
pub struct AbstractTooltipProps {
    //not full
    pub style: Option<String>, // React.CSSProperties;
    pub class_name: Option<String>,
    pub color: Option<()>,     // LiteralUnion<PresetColorType, string>;
    pub placement: Option<()>, // TooltipPlacement;
    pub builtin_placements: Option<()>, // typeof Placements;
    pub open_class_name: Option<String>,
    pub arrow_point_at_center: Option<()>,
    pub auto_adjust_overflow: Option<AdjustOverflowOrBool>, // boolean | AdjustOverflow;
    pub get_popup_container: Option<Callback<()>>, // (triggerNode: HTMLElement) => HTMLElement;
    pub children: Children,
}

// export interface AbstractTooltipProps extends Partial<Omit<RcTooltipProps, 'children'>> {
//   style?: React.CSSProperties;
//   className?: string;
//   color?: LiteralUnion<PresetColorType, string>;
//   placement?: TooltipPlacement;
//   builtinPlacements?: typeof Placements;
//   openClassName?: string;
//   arrowPointAtCenter?: boolean;
//   autoAdjustOverflow?: boolean | AdjustOverflow;
//   getPopupContainer?: (triggerNode: HTMLElement) => HTMLElement;
//   children?: React.ReactNode;
// }

pub type RenderFunction = fn() -> Html;
// export type RenderFunction = () => React.ReactNode;

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractTooltipPropsContent {
    ReactNode(Html),
    RenderFunction(RenderFunction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TooltipPropsWithOverlay {
    //not full
    pub style: Option<String>, // React.CSSProperties;
    pub class_name: Option<String>,
    pub color: Option<()>,     // LiteralUnion<PresetColorType, string>;
    pub placement: Option<()>, // TooltipPlacement;
    pub builtin_placements: Option<()>, // typeof Placements;
    pub open_class_name: Option<String>,
    pub arrow_point_at_center: Option<()>,
    pub auto_adjust_overflow: Option<()>, // boolean | AdjustOverflow;
    pub get_popup_container: Option<Callback<()>>, // (triggerNode: HTMLElement) => HTMLElement;
    pub children: Children,

    pub title: Option<AbstractTooltipPropsContent>,
    pub overlay: AbstractTooltipPropsContent,
}

// export interface TooltipPropsWithOverlay extends AbstractTooltipProps {
//   title?: React.ReactNode | RenderFunction;
//   overlay: React.ReactNode | RenderFunction;
// }

#[derive(Debug, PartialEq, Clone)]
pub struct TooltipPropsWithTitle {
    //not full
    pub style: Option<String>, // React.CSSProperties;
    pub class_name: Option<String>,
    pub color: Option<()>,     // LiteralUnion<PresetColorType, string>;
    pub placement: Option<()>, // TooltipPlacement;
    pub builtin_placements: Option<()>, // typeof Placements;
    pub open_class_name: Option<String>,
    pub arrow_point_at_center: Option<()>,
    pub auto_adjust_overflow: Option<()>, // boolean | AdjustOverflow;
    pub get_popup_container: Option<Callback<()>>, // (triggerNode: HTMLElement) => HTMLElement;
    pub children: Children,

    pub title: AbstractTooltipPropsContent,
    pub overlay: Option<AbstractTooltipPropsContent>,
}

// export interface TooltipPropsWithTitle extends AbstractTooltipProps {
//   title: React.ReactNode | RenderFunction;
//   overlay?: React.ReactNode | RenderFunction;
// }

#[derive(Debug, PartialEq, Clone)]
pub enum TooltipProps {
    WithTitle(TooltipPropsWithTitle),
    WithOverlay(TooltipPropsWithOverlay),
}

// export declare type TooltipProps = TooltipPropsWithTitle | TooltipPropsWithOverlay;

#[derive(Debug, PartialEq, Clone)]
pub struct SplittedObject {
    pub picked: PseudoCssWrapper,
    pub omitted: PseudoCssWrapper,
}

pub fn split_object(element: ElementType, omitted_keys_array: Vec<&str>) -> SplittedObject {
    let mut picked = PseudoCssWrapper {
        style: HashMap::<String, String>::new(),
    };
    let mut omitted = PseudoCssWrapper {
        style: HashMap::<String, String>::new(),
    };
    match element.clone().get_style_option() {
        None => (),
        Some(pseudo_css_wraper) => {
          for (style_key, style_value) in pseudo_css_wraper.style.clone() {
            for ommited_key in &omitted_keys_array {
                let k = style_key.clone();
                let v = style_value.clone();
                match style_key == ommited_key.to_string() {
                    true => {omitted.style.insert(k, v);},
                    false => {picked.style.insert(k, v);},
                }
            }
          };
        },
    }
    SplittedObject {
        picked, 
        omitted,
    }
}

// const splitObject = (obj: any, keys: string[]) => {
//   const picked: any = {};
//   const omitted: any = { ...obj };
//   keys.forEach(key => {
//     if (obj && key in obj) {
//       picked[key] = obj[key];
//       delete omitted[key];
//     }
//   });
//   return { picked, omitted };
// };
// const PresetColorRegex = new RegExp(`^(${PresetColorTypes.join('|')})(-inverse)?$`);

use crate::components::ant_design::button::ButtonProps;
use crate::components::ant_design::button::LoadingProp;

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchProps {
    pub disabled: Option<()>,         //dont know actually yet
    pub loading: Option<LoadingProp>, //dont know actually yet
    pub block: Option<()>,            //dont know actually yet

    pub style: Option<PseudoCssWrapper>
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElementType {
    Button(ButtonProps),
    Switch(SwitchProps),
    OtherDisabledCompatibleChildren(OtherDisabledCompatibleChildrenProps),
}

impl ElementType {
    pub fn get_html(&self) -> Html {
        html!{}
    }
    pub fn get_style_option(&self) -> Option<PseudoCssWrapper> {
        match self {
            ElementType::Button(props) => props.style.clone(),
            ElementType::Switch(props) => props.style.clone(),
            ElementType::OtherDisabledCompatibleChildren(props) => props.style.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OtherDisabledCompatibleChildrenProps {
    pub block: Option<()>,
    pub style: Option<PseudoCssWrapper>
}

impl OtherDisabledCompatibleChildrenProps {
    pub fn get_html(&self, style: String, class: String, children: Html) -> Html {
        html!{
          <span
            style={style.clone()}
            class={class.clone()}
                // classNames(element.props.className, `${prefixCls}-disabled-compatible-wrapper`)}
          >
            {children}//{child}
          </span>
        }
    }
}

#[function_component(OtherDisabledCompatibleChildren)]
pub fn other_disabled_compatible_children() -> Html {
    html! {}
}

pub fn get_disabled_compatible_children(element: ElementType, prefix_cls: String) -> ElementType {
    let should_return_something_else = match element.clone() {
        ElementType::Button(props) => match props.disabled {
            None => false,
            Some(_) => true,
        },
        ElementType::Switch(props) => match (props.disabled, props.loading) {
            (None, None) => false,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(_), Some(_)) => true,
        },
        ElementType::OtherDisabledCompatibleChildren(_) => false,
    };
    match should_return_something_else {
        false => element,
        true => {
            let block = match element.clone() {
                ElementType::Button(props) => props.block,
                ElementType::Switch(props) => props.block,
                ElementType::OtherDisabledCompatibleChildren(props) => props.block,
            };
            let width = match block {
                None => String::from("null"),
                Some(_) => String::from("100%"),
            };
            let omitted_str_array = vec![
                  "position",
                  "left",
                  "right",
                  "top",
                  "bottom",
                  "float",
                  "display",
                  "zIndex",
            ];
            
            // Pick some layout related style properties up to span
            // Prevent layout bugs like https://github.com/ant-design/ant-design/issues/5254
            // const { picked, omitted } = splitObject(element.props.style, [
            //   'position',
            //   'left',
            //   'right',
            //   'top',
            //   'bottom',
            //   'float',
            //   'display',
            //   'zIndex',
            // ]);
            
            let splitted_object = split_object(element.clone(), omitted_str_array);

            // const splitObject = (obj: any, keys: string[]) => {
            //   const picked: any = {};
            //   const omitted: any = { ...obj };
            //   keys.forEach(key => {
            //     if (obj && key in obj) {
            //       picked[key] = obj[key];
            //       delete omitted[key];
            //     }
            //   });
            //   return { picked, omitted };
            // };
            //
            let span_style = format!(
                "display: inline-block; cursor: not-allowed; width: {}; {}",
                width,
                splitted_object.picked.to_string()
            );
            let button_style = format!("pointerEvents: none; {}", splitted_object.omitted.to_string());
            // const buttonStyle = {
            //   ...omitted,
            //   pointerEvents: 'none',
            // };
            let child = match element.clone() {
                ElementType::Button(props) => ElementType::Button(ButtonProps{
                    disabled: props.disabled,
                    ghost: props.ghost,
                    href: props.href,
                    html_type: props.html_type,
                    icon: props.icon,
                    loading: props.loading,
                    shape: props.shape,
                    size: props.size,
                    target: props.target,
                    button_type: props.button_type,
                    on_click: props.on_click,
                    block: props.block,
                    placeholder: props.placeholder,
                    style: props.style,
                }),
                ElementType::Switch(props) => ElementType::Switch(SwitchProps{
                    disabled: props.disabled,
                    loading: props.loading,
                    block: props.block,
                    style: props.style,
                }),
                ElementType::OtherDisabledCompatibleChildren(props) => ElementType::OtherDisabledCompatibleChildren(OtherDisabledCompatibleChildrenProps{
                    block: props.block,
                    style: props.style,
                }),
            };
            // const child = cloneElement(element, {
            //   style: buttonStyle,
            //   className: null,
            // });
            // return (
            //   <span
            //     style={spanStyle}
            //     className={classNames(element.props.className, `${prefixCls}-disabled-compatible-wrapper`)}
            //   >
            //     {child}
            //   </span>
            // );
            ElementType::OtherDisabledCompatibleChildren(OtherDisabledCompatibleChildrenProps{
                block: None,
                style: None,
            })
        }
    }
}

// // Fix Tooltip won't hide at disabled button
// // mouse events don't trigger at disabled button in Chrome
// // https://github.com/react-component/tooltip/issues/18
// function getDisabledCompatibleChildren(element: React.ReactElement<any>, prefixCls: string) {
//   const elementType = element.type as any;
//   if (
//     ((elementType.__ANT_BUTTON === true || element.type === 'button') && element.props.disabled) ||
//     (elementType.__ANT_SWITCH === true && (element.props.disabled || element.props.loading))
//   ) {
//     // Pick some layout related style properties up to span
//     // Prevent layout bugs like https://github.com/ant-design/ant-design/issues/5254
//     const { picked, omitted } = splitObject(element.props.style, [
//       'position',
//       'left',
//       'right',
//       'top',
//       'bottom',
//       'float',
//       'display',
//       'zIndex',
//     ]);
//     const spanStyle = {
//       display: 'inline-block', // default inline-block is important
//       ...picked,
//       cursor: 'not-allowed',
//       width: element.props.block ? '100%' : null,
//     };
//     const buttonStyle = {
//       ...omitted,
//       pointerEvents: 'none',
//     };
//     const child = cloneElement(element, {
//       style: buttonStyle,
//       className: null,
//     });
//     return (
//       <span
//         style={spanStyle}
//         className={classNames(element.props.className, `${prefixCls}-disabled-compatible-wrapper`)}
//       >
//         {child}
//       </span>
//     );
//   }
//   return element;
// }

// const Tooltip = React.forwardRef<unknown, TooltipProps>((props, ref) => {
//   const {
//     getPopupContainer: getContextPopupContainer,
//     getPrefixCls,
//     direction,
//   } = React.useContext(ConfigContext);

//   const [visible, setVisible] = useMergedState(false, {
//     value: props.visible,
//     defaultValue: props.defaultVisible,
//   });

//   const isNoTitle = () => {
//     const { title, overlay } = props;
//     return !title && !overlay && title !== 0; // overlay for old version compatibility
//   };

//   const onVisibleChange = (vis: boolean) => {
//     setVisible(isNoTitle() ? false : vis);

//     if (!isNoTitle()) {
//       props.onVisibleChange?.(vis);
//     }
//   };

//   const getTooltipPlacements = () => {
//     const { builtinPlacements, arrowPointAtCenter, autoAdjustOverflow } = props;
//     return (
//       builtinPlacements ||
//       getPlacements({
//         arrowPointAtCenter,
//         autoAdjustOverflow,
//       })
//     );
//   };

//   // 动态设置动画点
//   const onPopupAlign = (domNode: HTMLElement, align: any) => {
//     const placements: any = getTooltipPlacements();
//     // 当前返回的位置
//     const placement = Object.keys(placements).find(
//       key =>
//         placements[key].points[0] === align.points[0] &&
//         placements[key].points[1] === align.points[1],
//     );
//     if (!placement) {
//       return;
//     }
//     // 根据当前坐标设置动画点
//     const rect = domNode.getBoundingClientRect();
//     const transformOrigin = {
//       top: '50%',
//       left: '50%',
//     };
//     if (placement.indexOf('top') >= 0 || placement.indexOf('Bottom') >= 0) {
//       transformOrigin.top = `${rect.height - align.offset[1]}px`;
//     } else if (placement.indexOf('Top') >= 0 || placement.indexOf('bottom') >= 0) {
//       transformOrigin.top = `${-align.offset[1]}px`;
//     }
//     if (placement.indexOf('left') >= 0 || placement.indexOf('Right') >= 0) {
//       transformOrigin.left = `${rect.width - align.offset[0]}px`;
//     } else if (placement.indexOf('right') >= 0 || placement.indexOf('Left') >= 0) {
//       transformOrigin.left = `${-align.offset[0]}px`;
//     }
//     domNode.style.transformOrigin = `${transformOrigin.left} ${transformOrigin.top}`;
//   };

//   const getOverlay = () => {
//     const { title, overlay } = props;
//     if (title === 0) {
//       return title;
//     }
//     return overlay || title || '';
//   };

//   const { getPopupContainer, ...otherProps } = props;

//   const {
//     prefixCls: customizePrefixCls,
//     openClassName,
//     getTooltipContainer,
//     overlayClassName,
//     color,
//     overlayInnerStyle,
//     children,
//   } = props;
//   const prefixCls = getPrefixCls('tooltip', customizePrefixCls);
//   const rootPrefixCls = getPrefixCls();

//   let tempVisible = visible;
//   // Hide tooltip when there is no title
//   if (!('visible' in props) && isNoTitle()) {
//     tempVisible = false;
//   }

//   const child = getDisabledCompatibleChildren(
//     isValidElement(children) ? children : <span>{children}</span>,
//     prefixCls,
//   );
//   const childProps = child.props;
//   const childCls = classNames(childProps.className, {
//     [openClassName || `${prefixCls}-open`]: true,
//   });

//   const customOverlayClassName = classNames(overlayClassName, {
//     [`${prefixCls}-rtl`]: direction === 'rtl',
//     [`${prefixCls}-${color}`]: color && PresetColorRegex.test(color),
//   });

//   let formattedOverlayInnerStyle = overlayInnerStyle;
//   let arrowContentStyle;
//   if (color && !PresetColorRegex.test(color)) {
//     formattedOverlayInnerStyle = { ...overlayInnerStyle, background: color };
//     // @ts-ignore
//     arrowContentStyle = { '--antd-arrow-background-color': color };
//   }

//   return (
//     <RcTooltip
//       {...otherProps}
//       prefixCls={prefixCls}
//       overlayClassName={customOverlayClassName}
//       getTooltipContainer={getPopupContainer || getTooltipContainer || getContextPopupContainer}
//       ref={ref}
//       builtinPlacements={getTooltipPlacements()}
//       overlay={getOverlay()}
//       visible={tempVisible}
//       onVisibleChange={onVisibleChange}
//       onPopupAlign={onPopupAlign}
//       overlayInnerStyle={formattedOverlayInnerStyle}
//       arrowContent={<span className={`${prefixCls}-arrow-content`} style={arrowContentStyle} />}
//       motion={{
//         motionName: getTransitionName(rootPrefixCls, 'zoom-big-fast', props.transitionName),
//         motionDeadline: 1000,
//       }}
//     >
//       {tempVisible ? cloneElement(child, { className: childCls }) : child}
//     </RcTooltip>
//   );
// });

// if (process.env.NODE_ENV !== 'production') {
//   Tooltip.displayName = 'Tooltip';
// }

// Tooltip.defaultProps = {
//   placement: 'top' as TooltipPlacement,
//   mouseEnterDelay: 0.1,
//   mouseLeaveDelay: 0.1,
//   arrowPointAtCenter: false,
//   autoAdjustOverflow: true,
// };

// export default Tooltip;
