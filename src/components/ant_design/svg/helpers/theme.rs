#[derive(PartialEq)]
pub enum Theme {
    Filled,
    Outlined,
    TwoTone,
}

impl Default for &Theme {
    fn default() -> Self {
        &Theme::Outlined
    }
}