use colorsys::Hsl;

#[derive(Debug, PartialEq, Clone)]
pub enum FillWith {
    Hsl(Hsl),
    CurrentColor,
}

impl FillWith {
    pub fn into_string_color(&self) -> String {
        match self {
            FillWith::Hsl(hsl) => hsl.to_css_string(),
            FillWith::CurrentColor => String::from("currentColor"),
        }
    }
}
