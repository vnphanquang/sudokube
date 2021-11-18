pub mod colors;
pub mod key_binding;

use crate::config::colors::ColorsConfig;
use crate::config::key_binding::KeyBinding;
use crate::lib::merge::strategy::{merge_hash_map, merge_nested_struct};
use dirs::home_dir;
use merge::Merge;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use serde_yaml::{from_str, to_string};
use std::collections::HashMap;
use std::fs;

#[serde_as]
#[derive(Merge, Debug, Serialize, Deserialize)]
#[serde(default = "Config::blank")]
pub struct Config {
    context_highlight: Option<bool>,

    #[merge(strategy = merge_nested_struct)]
    colors: Option<ColorsConfig>,

    #[merge(strategy = merge_hash_map)]
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    value_map: Option<HashMap<u8, String>>,

    #[merge(strategy = merge_nested_struct)]
    key_binding: Option<KeyBinding>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            context_highlight: Some(Self::default_context_highlight()),
            colors: Some(ColorsConfig::default()),
            value_map: Some(Self::default_value_map()),
            key_binding: Some(KeyBinding::default()),
        }
    }
}

// public
impl Config {
    pub fn blank() -> Self {
        Self {
            context_highlight: None,
            colors: None,
            value_map: None,
            key_binding: None,
        }
    }

    #[inline]
    pub fn default_path() -> String {
        let mut buf = home_dir().unwrap_or_default();
        buf.push(".sudokuberc.yaml");
        buf.to_string_lossy().into_owned()
    }

    pub fn read(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(yaml) => Config::from_yaml(&yaml),
            Err(_) => Config::blank(),
        }
    }

    pub fn write(&self, path: &str) -> Result<(), std::io::Error> {
        fs::write(path, self.to_yaml())?;
        Ok(())
    }

    pub fn toggle_context_highlight(&mut self) {
        if let Some(b) = self.context_highlight {
            self.context_highlight = Some(!b);
        }
    }
}

// getter
impl Config {
    pub fn context_highlight(&self) -> bool {
        self.context_highlight
            .unwrap_or(Self::default_context_highlight())
    }

    pub fn colors(&self) -> ColorsConfig {
        self.colors.unwrap_or(ColorsConfig::default())
    }

    pub fn value_map(&self) -> HashMap<u8, String> {
        self.value_map.clone().unwrap_or(Self::default_value_map())
    }

    pub fn key_binding(&self) -> KeyBinding {
        self.key_binding.unwrap_or(KeyBinding::default())
    }
}

// private
impl Config {
    fn default_value_map() -> HashMap<u8, String> {
        HashMap::from([
            (0, String::from("1")),
            (1, String::from("2")),
            (2, String::from("3")),
            (3, String::from("4")),
            (4, String::from("5")),
            (5, String::from("6")),
            (6, String::from("7")),
            (7, String::from("8")),
            (8, String::from("9")),
        ])
    }

    fn default_context_highlight() -> bool {
        true
    }

    fn from_yaml(yaml: &str) -> Self {
        match from_str(yaml) {
            Ok(config) => config,
            Err(error) => {
                println!("Error parsing config file: {}", error);
                Config::blank()
            }
        }
    }

    fn to_yaml(&self) -> String {
        match to_string(self) {
            Ok(yaml) => yaml,
            Err(error) => {
                println!("Error serializing config file: {}", error);
                String::default()
            }
        }
    }
}
