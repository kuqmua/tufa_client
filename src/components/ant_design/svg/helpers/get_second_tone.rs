use colorsys::{Rgb, Hsl};

pub fn get_second_tone(color: String) -> String {
    match Rgb::from_hex_str(&color) {//optimize it later?
      Err(_) => String::from("hsl(0, 100%, 100%)"),
      Ok(rgb) => {
        let hsl = Hsl::from(&rgb);
        format!("hsl({}, {}%, 95%)", hsl.hue(), hsl.saturation())
      },
    }
}