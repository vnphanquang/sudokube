pub mod base;
mod navigation;

use crate::config::key_binding::base::KeyDefinition;
use crate::config::key_binding::navigation::NavigationKeyBinding;
use crossterm::event::KeyCode;
use merge::Merge;
use serde::{Deserialize, Serialize};

use self::base::KeyModifier;

#[derive(Merge, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default = "KeyBinding::blank")]
pub struct KeyBinding {
    #[merge(strategy = merge::option::recurse)]
    navigation: Option<NavigationKeyBinding>,
    #[merge(strategy = crate::lib::merge::strategy::option::overwrite)]
    toggle_context_highlight: Option<KeyDefinition>,
    #[merge(strategy = crate::lib::merge::strategy::option::overwrite)]
    delete: Option<KeyDefinition>,
}

impl Default for KeyBinding {
    fn default() -> Self {
        Self {
            navigation: Some(NavigationKeyBinding::default()),
            toggle_context_highlight: Some(KeyBinding::default_toggle_context_highlight()),
            delete: Some(KeyBinding::default_delete()),
        }
    }
}

impl KeyBinding {
    pub fn blank() -> Self {
        Self {
            navigation: None,
            toggle_context_highlight: None,
            delete: None,
        }
    }

    pub fn navigation(&self) -> NavigationKeyBinding {
        self.navigation.unwrap_or(NavigationKeyBinding::default())
    }

    pub fn delete(&self) -> KeyDefinition {
        self.delete.unwrap_or(KeyBinding::default_delete())
    }

    pub fn toggle_context_highlight(&self) -> KeyDefinition {
        self.toggle_context_highlight
            .unwrap_or(KeyBinding::default_toggle_context_highlight())
    }
}
impl KeyBinding {
    fn default_toggle_context_highlight() -> KeyDefinition {
        KeyDefinition {
            code: Some(KeyCode::Char('H')),
            modifier: Some(KeyModifier::Shift),
        }
    }

    fn default_delete() -> KeyDefinition {
        KeyDefinition {
            code: Some(KeyCode::Char('x')),
            modifier: None,
        }
    }
}
