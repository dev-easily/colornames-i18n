use colors_transform::{Color, Rgb};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct OriginColor {
    pub en_name: String,
    pub hex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct I18nColor {
    pub hex: String,
    pub rgb: String,
    pub hsl: String,
    // #[serde_as(as = "Vec<(_, _)>")]
    pub names: HashMap<String, Vec<String>>
}
impl From<OriginColor> for I18nColor {
    fn from(value: OriginColor) -> Self {
        let rgb = Rgb::from_hex_str(&value.hex[1..]).unwrap();
        let hsl = rgb.to_hsl();
        I18nColor {
            hex: value.hex,
            rgb: rgb.to_css_string(),
            hsl: hsl.to_css_string(),
            names: HashMap::new(),
        }
    }
}

impl I18nColor {
    pub fn new(en_name: String, hex: String) -> Self {
        Self::from(OriginColor {
            en_name,
            hex
        })
    }

    pub fn with_names(self, names: HashMap<String, Vec<String>>) -> Self {
        Self {
            hex: self.hex,
            rgb: self.rgb,
            hsl: self.hsl,
            names,
        }
    }
}
