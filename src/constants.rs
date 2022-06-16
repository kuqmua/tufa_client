use colorsys::Hsl;
use lazy_static::lazy_static;

pub const PROJECT_NAME: &str = "tufa_client";
pub const HEADER_ICONS_COLOR: &str = "#5B6267";
pub const FEED_ICONS_COLOR: &str = "#5B6267";
pub const INTERFACE_LINES_COLOR: &str = "#222c36";
pub const FEED_BUTTONS_BACKGROUND_COLOR: &str = "#222c36";
pub const ACTIVE_COLOR: &str = "#ffffa2";
pub const BACKGROUND_COLOR: &str = "#16202A";
pub const SHADOW_COLOR: &str = "#000000";
pub const FEED_WIDTH_PX: u32 = 470;
pub const DEFAULT_PADDING_PX: u32 = 8;
pub const HEADER_HEIGHT_PX: u32 = 42;
pub const HEADER_BORDER_BOTTOM_PX: u32 = 1;

lazy_static! {
    pub static ref WHITE_HSL: Hsl = Hsl::new(0.0, 100.0, 100.0, Some(1.0));
}