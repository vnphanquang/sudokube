use crate::enums::RenderVariant;
use merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);
impl Rgb {
    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }
}

impl Merge for Rgb {
    fn merge(&mut self, other: Self) {
        self.0 = other.0;
        self.1 = other.1;
        self.2 = other.2;
    }
}

#[derive(Merge, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default)]
pub struct DisplayColor {
    pub bg: Option<Rgb>,
    pub color: Option<Rgb>,
}
impl Default for DisplayColor {
    fn default() -> Self {
        DisplayColor {
            bg: None,
            color: None,
        }
    }
}

#[derive(Merge, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default = "ColorsConfig::blank")]
pub struct ColorsConfig {
    fixed: Option<DisplayColor>,
    error: Option<DisplayColor>,
    directional_relative: Option<DisplayColor>,
    same_value: Option<DisplayColor>,
    default: Option<DisplayColor>,
}
impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            fixed: Some(Self::default_fixed()),
            error: Some(Self::default_error()),
            directional_relative: Some(Self::default_directional_relative()),
            same_value: Some(Self::default_same_value()),
            default: Some(Self::default_default()),
        }
    }
}
impl ColorsConfig {
    pub fn blank() -> Self {
        Self {
            fixed: None,
            error: None,
            directional_relative: None,
            same_value: None,
            default: None,
        }
    }

    pub fn get(&self, variant: RenderVariant) -> DisplayColor {
        match variant {
            RenderVariant::Fixed => self.fixed.unwrap_or(Self::default_fixed()),
            RenderVariant::Error => self.error.unwrap_or(Self::default_error()),
            RenderVariant::DirectionalRelative => self
                .directional_relative
                .unwrap_or(Self::default_directional_relative()),
            RenderVariant::SameValue => self.same_value.unwrap_or(Self::default_same_value()),
            RenderVariant::Default => self.default.unwrap_or(Self::default_default()),
        }
    }
}

impl ColorsConfig {
    fn default_fixed() -> DisplayColor {
        DisplayColor {
            bg: None,
            color: Some(Rgb(0, 255, 0)),
        }
    }

    fn default_error() -> DisplayColor {
        DisplayColor {
            bg: Some(Rgb(255, 0, 0)),
            color: Some(Rgb(255, 255, 255)),
        }
    }

    fn default_directional_relative() -> DisplayColor {
        DisplayColor {
            bg: Some(Rgb(78, 78, 78)),
            color: None,
        }
    }

    fn default_same_value() -> DisplayColor {
        DisplayColor {
            bg: None,
            color: Some(Rgb(0, 215, 255)),
        }
    }

    fn default_default() -> DisplayColor {
        DisplayColor::default()
    }
}
