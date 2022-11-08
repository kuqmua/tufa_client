use crate::components::rc::rc_animate::util::motion::{can_use_dom, get_vendor_prefixes, Prefixes};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static VENDOR_PREFIXES: Lazy<Prefixes> =
    Lazy::new(|| get_vendor_prefixes(can_use_dom(), HashMap::new()));
