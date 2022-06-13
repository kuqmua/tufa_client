#[derive(Debug, PartialEq)]
pub struct Rotate {
    degrees: u16
}

impl Rotate {
    pub fn change(possible_degrees: u16) -> Result<Self, u16> {
        if possible_degrees > 360 {
            return Err(possible_degrees);
        }
        Ok(Self {
            degrees: possible_degrees
        })
    }
}