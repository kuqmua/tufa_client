use crate::components::rc::rc_animate::util::motion::get_option_style;
use once_cell::sync::Lazy;

pub static STYLE: Lazy<Option<bool>> = Lazy::new(get_option_style);
