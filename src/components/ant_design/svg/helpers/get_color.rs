use colorsys::Hsl;
use crate::constants::WHITE_HSL;

pub fn get_color(option_color: Option<Hsl>) -> Hsl {
    match option_color {
        None => WHITE_HSL.clone(),//as default
        Some(fill) => fill,
    }
}