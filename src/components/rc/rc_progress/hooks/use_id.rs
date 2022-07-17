use lazy_static::lazy_static;
use crate::components::rc::rc_animate::util::motion::can_use_dom;
use std::sync::Mutex;

// import * as React from 'react';
// import canUseDom from 'rc-util/lib/Dom/canUseDom';

lazy_static! {
    pub static ref UUID: Mutex<i32> = Mutex::new(0);
}

// pub const UUID: i32 = 0;
// let uuid = 0;

lazy_static! {
    pub static ref IS_BROWSER_CLIENT: bool = can_use_dom();
}

// /** Is client side and not jsdom */
// export const isBrowserClient = process.env.NODE_ENV !== 'test' && canUseDom();

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UUIDStruct {
    Number(i32),
    String(String),
}

impl UUIDStruct {
  pub fn to_string(&self) -> String {
    match self {
        UUIDStruct::Number(n) => n.to_string(),
        UUIDStruct::String(s) => s.clone(),
    }
  }
}

pub fn get_uuid() -> UUIDStruct {
  let ret_id: UUIDStruct;
  if *IS_BROWSER_CLIENT {
    ret_id = UUIDStruct::Number(*UUID.lock().unwrap());
    *UUID.lock().unwrap() += 1;
  }
  else {
    ret_id = UUIDStruct::String(String::from("TEST_OR_SSR'"));
  }
  ret_id
}

// /** Get unique id for accessibility usage */
// function getUUID(): number | string {
//   let retId: string | number;

//   // Test never reach
//   /* istanbul ignore if */
//   if (isBrowserClient) {
//     retId = uuid;
//     uuid += 1;
//   } else {
//     retId = 'TEST_OR_SSR';
//   }

//   return retId;
// }

pub fn default(id: Option<String>) -> String {
  let inner_id = String::from("");
  // let set_inner_id = format!("rc_progress_{}", get_uuid().to_string());

  
  // Inner id for accessibility usage. Only work in client side
  // const [innerId, setInnerId] = React.useState<string>();
  // React.useEffect(() => {
  //   setInnerId(`rc_progress_${getUUID()}`);
  // }, []);
  match id {
    None => inner_id,
    Some(s) => s,
  }
  // return id || innerId;
}

// export default (id?: string) => {
//   // Inner id for accessibility usage. Only work in client side
//   const [innerId, setInnerId] = React.useState<string>();
//   React.useEffect(() => {
//     setInnerId(`rc_progress_${getUUID()}`);
//   }, []);
//   return id || innerId;
// };
