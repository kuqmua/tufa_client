use std::collections::HashMap;
use convert_case::Case;
use convert_case::Casing;

// const canUseDOM = !!(typeof window !== 'undefined' && window.document && window.document.createElement);

pub fn make_prefix_map(style_prop: String, event_name: String) -> HashMap::<String, String> {
  let mut prefixes = HashMap::<String, String>::new();
  prefixes.insert(style_prop.to_case(Case::Lower), event_name.to_case(Case::Lower));
  prefixes.insert(format!("Webkit{}", style_prop), format!("webkit{}", event_name));
  prefixes.insert(format!("Moz{}", style_prop), format!("moz{}", event_name));
  prefixes.insert(format!("ms{}", style_prop), format!("MS{}", event_name));
  prefixes.insert(format!("O{}", style_prop), format!("o{}", event_name));
  return prefixes;
}

// // ================= Transition =================
// // Event wrapper. Copy from react source code
// function makePrefixMap(styleProp, eventName) {
//   const prefixes = {};

//   prefixes[styleProp.toLowerCase()] = eventName.toLowerCase();
//   prefixes[`Webkit${styleProp}`] = `webkit${eventName}`;
//   prefixes[`Moz${styleProp}`] = `moz${eventName}`;
//   prefixes[`ms${styleProp}`] = `MS${eventName}`;
//   prefixes[`O${styleProp}`] = `o${eventName.toLowerCase()}`;

//   return prefixes;
// }

// export function getVendorPrefixes(domSupport, win) {
//   const prefixes = {
//     animationend: makePrefixMap('Animation', 'AnimationEnd'),
//     transitionend: makePrefixMap('Transition', 'TransitionEnd'),
//   };

//   if (domSupport) {
//     if (!('AnimationEvent' in win)) {
//       delete prefixes.animationend.animation;
//     }

//     if (!('TransitionEvent' in win)) {
//       delete prefixes.transitionend.transition;
//     }
//   }

//   return prefixes;
// }

// const vendorPrefixes = getVendorPrefixes(canUseDOM, typeof window !== 'undefined' ? window : {});

// let style = {};

// if (canUseDOM) {
//   style = document.createElement('div').style;
// }

// const prefixedEventNames = {};

// export function getVendorPrefixedEventName(eventName) {
//   if (prefixedEventNames[eventName]) {
//     return prefixedEventNames[eventName];
//   }

//   const prefixMap = vendorPrefixes[eventName];

//   if (prefixMap) {
//     const stylePropList = Object.keys(prefixMap);
//     const len = stylePropList.length;
//     for (let i = 0; i < len; i += 1) {
//       const styleProp = stylePropList[i];
//       if (Object.prototype.hasOwnProperty.call(prefixMap, styleProp) && styleProp in style) {
//         prefixedEventNames[eventName] = prefixMap[styleProp];
//         return prefixedEventNames[eventName];
//       }
//     }
//   }

//   return '';
// }

// export const animationEndName = getVendorPrefixedEventName('animationend');
// export const transitionEndName = getVendorPrefixedEventName('transitionend');
// export const supportTransition = !!(animationEndName && transitionEndName);

// export function getTransitionName(transitionName, transitionType) {
//   if (!transitionName) return null;

//   if (typeof transitionName === 'object') {
//     const type = transitionType.replace(/-\w/g, (match) => match[1].toUpperCase());
//     return transitionName[type];
//   }

//   return `${transitionName}-${transitionType}`;
// }