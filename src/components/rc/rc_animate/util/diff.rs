use std::collections::HashMap;

pub const STATUS_ADD: &str = "add";
pub const STATUS_KEEP: &str = "keep";
pub const STATUS_REMOVE: &str = "remove";
pub const STATUS_REMOVED: &str = "removed";

// export const STATUS_ADD = 'add';
// export const STATUS_KEEP = 'keep';
// export const STATUS_REMOVE = 'remove';
// export const STATUS_REMOVED = 'removed';

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectOrString {
    Object(Object),
    String(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Object {
    pub key: String,
}

pub fn wrap_key_to_object(key: ObjectOrString) -> HashMap<String, String>{
  let mut key_obj: HashMap<String, String> = HashMap::new();
  match key {
    ObjectOrString::Object(hs) => {
        key_obj.insert(String::from("key"), hs.key);
    },
    ObjectOrString::String(s) => {
        key_obj.insert(String::from("key"), s);
    }
}
key_obj
}

// export function wrapKeyToObject(key) {
//   let keyObj;
//   if (key && typeof key === 'object' && 'key' in key) {
//     keyObj = key;
//   } else {
//     keyObj = { key };
//   }
//   return {
//     ...keyObj,
//     key: String(keyObj.key),
//   };
// }

pub fn parse_keys(keys: Vec<ObjectOrString>) -> Vec<HashMap<String, String>>{
  let mut vec = Vec::new(); 
  for key in keys {
    vec.push(wrap_key_to_object(key));
  }
  vec
}

// export function parseKeys(keys = []) {
//   return keys.map(wrapKeyToObject);
// }

// export function diffKeys(prevKeys = [], currentKeys = []) {
//   let list = [];
//   let currentIndex = 0;
//   const currentLen = currentKeys.length;

//   const prevKeyObjects = parseKeys(prevKeys);
//   const currentKeyObjects = parseKeys(currentKeys);

//   // Check prev keys to insert or keep
//   prevKeyObjects.forEach(keyObj => {
//     let hit = false;

//     for (let i = currentIndex; i < currentLen; i += 1) {
//       const currentKeyObj = currentKeyObjects[i];
//       if (currentKeyObj.key === keyObj.key) {
//         // New added keys should add before current key
//         if (currentIndex < i) {
//           list = list.concat(
//             currentKeyObjects.slice(currentIndex, i).map(obj => ({ ...obj, status: STATUS_ADD })),
//           );
//           currentIndex = i;
//         }
//         list.push({
//           ...currentKeyObj,
//           status: STATUS_KEEP,
//         });
//         currentIndex += 1;

//         hit = true;
//         break;
//       }
//     }

//     // If not hit, it means key is removed
//     if (!hit) {
//       list.push({
//         ...keyObj,
//         status: STATUS_REMOVE,
//       });
//     }
//   });

//   // Add rest to the list
//   if (currentIndex < currentLen) {
//     list = list.concat(
//       currentKeyObjects.slice(currentIndex).map(obj => ({ ...obj, status: STATUS_ADD })),
//     );
//   }

//   /**
//    * Merge same key when it remove and add again:
//    *    [1 - add, 2 - keep, 1 - remove] -> [1 - keep, 2 - keep]
//    */
//   const keys = {};
//   list.forEach(({ key }) => {
//     keys[key] = (keys[key] || 0) + 1;
//   });
//   const duplicatedKeys = Object.keys(keys).filter(key => keys[key] > 1);
//   duplicatedKeys.forEach(matchKey => {
//     // Remove `STATUS_REMOVE` node.
//     list = list.filter(({ key, status }) => key !== matchKey || status !== STATUS_REMOVE);

//     // Update `STATUS_ADD` to `STATUS_KEEP`
//     list.forEach(node => {
//       if (node.key === matchKey) {
//         node.status = STATUS_KEEP;
//       }
//     });
//   });

//   return list;
// }