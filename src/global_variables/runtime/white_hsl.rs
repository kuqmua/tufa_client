use colorsys::Hsl;
use once_cell::sync::Lazy;

pub static WHITE_HSL: Lazy<Hsl> = Lazy::new(|| Hsl::new(0.0, 100.0, 100.0, Some(1.0)));
