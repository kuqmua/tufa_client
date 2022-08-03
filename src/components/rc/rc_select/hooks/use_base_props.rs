use std::collections::HashMap;
use web_sys::FocusEvent;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Children;
use yew::Event;
use yew::Html;
use yew::NodeRef;
use yew::Properties;

// /**
//  * BaseSelect provide some parsed data into context.
//  * You can use this hooks to get them.
//  */
// import * as React from 'react';
// import type { BaseSelectProps } from '../BaseSelect';

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct BaseSelectContextProps {
    pub trigger_open: Option<()>,
    pub multiple: Option<()>,
    pub toggle_open: Option<Callback<()>>, //(open?: boolean) => void
}

// export interface BaseSelectContextProps extends BaseSelectProps {
//   triggerOpen: boolean;
//   multiple: boolean;
//   toggleOpen: (open?: boolean) => void;
// }

// export const BaseSelectContext = React.createContext<BaseSelectContextProps>(null);

// export default function useBaseProps() {
//   return React.useContext(BaseSelectContext);
// }
