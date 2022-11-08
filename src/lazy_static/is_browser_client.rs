use crate::components::rc::rc_animate::util::motion::can_use_dom;
use once_cell::sync::Lazy;

pub static IS_BROWSER_CLIENT: Lazy<bool> = Lazy::new(can_use_dom);
