pub mod base;
mod navigation;

use crate::config::key_binding::base::{KeyDefinition, KeyModifier};
use crate::config::key_binding::navigation::NavigationKeyBinding;
use crate::lib::merge::strategy::merge_nested_struct;
use merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(Merge, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default = "KeyBinding::blank")]
pub struct KeyBinding {
    #[merge(strategy = merge_nested_struct)]
    navigation: Option<NavigationKeyBinding>,
    toggle_context_highlight: Option<KeyDefinition>,
    delete: Option<KeyDefinition>,
    quit: Option<KeyDefinition>,
}

impl Default for KeyBinding {
    fn default() -> Self {
        Self {
            navigation: Some(NavigationKeyBinding::default()),
            toggle_context_highlight: Some(KeyBinding::default_toggle_context_highlight()),
            delete: Some(KeyBinding::default_delete()),
            quit: Some(KeyBinding::default_quit()),
        }
    }
}

impl KeyBinding {
    pub fn blank() -> Self {
        Self {
            navigation: None,
            toggle_context_highlight: None,
            delete: None,
            quit: None,
        }
    }

    pub fn navigation(&self) -> NavigationKeyBinding {
        self.navigation.unwrap_or(NavigationKeyBinding::default())
    }

    pub fn delete(&self) -> KeyDefinition {
        self.delete.unwrap_or(KeyBinding::default_delete())
    }

    pub fn quit(&self) -> KeyDefinition {
        self.quit.unwrap_or(KeyBinding::default_quit())
    }

    pub fn toggle_context_highlight(&self) -> KeyDefinition {
        self.toggle_context_highlight
            .unwrap_or(KeyBinding::default_toggle_context_highlight())
    }
}
impl KeyBinding {
    fn default_toggle_context_highlight() -> KeyDefinition {
        KeyDefinition {
            key: Some('H'),
            modifier: Some(KeyModifier::Shift),
        }
    }

    fn default_delete() -> KeyDefinition {
        KeyDefinition {
            key: Some('x'),
            modifier: None,
        }
    }

    fn default_quit() -> KeyDefinition {
        KeyDefinition {
            key: Some('q'),
            modifier: None,
        }
    }
}
