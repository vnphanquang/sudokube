use crate::config::key_binding::base::{KeyDefinition, KeyModifier};
use merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(Merge, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default = "NavigationKeyBinding::blank")]
pub struct NavigationKeyBinding {
    left: Option<KeyDefinition>,
    down: Option<KeyDefinition>,
    right: Option<KeyDefinition>,
    up: Option<KeyDefinition>,
    next_group: Option<KeyDefinition>,
    previous_group: Option<KeyDefinition>,
}
impl Default for NavigationKeyBinding {
    fn default() -> Self {
        Self {
            left: Some(Self::default_left()),
            down: Some(Self::default_down()),
            up: Some(Self::default_up()),
            right: Some(Self::default_right()),
            next_group: Some(Self::default_next_group()),
            previous_group: Some(Self::default_previous_group()),
        }
    }
}
impl NavigationKeyBinding {
    pub fn blank() -> Self {
        Self {
            left: None,
            down: None,
            right: None,
            up: None,
            next_group: None,
            previous_group: None,
        }
    }

    pub fn left(&self) -> KeyDefinition {
        self.left.unwrap_or(NavigationKeyBinding::default_left())
    }
    pub fn right(&self) -> KeyDefinition {
        self.right.unwrap_or(NavigationKeyBinding::default_right())
    }
    pub fn up(&self) -> KeyDefinition {
        self.up.unwrap_or(NavigationKeyBinding::default_up())
    }
    pub fn down(&self) -> KeyDefinition {
        self.down.unwrap_or(NavigationKeyBinding::default_down())
    }
    pub fn next_group(&self) -> KeyDefinition {
        self.next_group
            .unwrap_or(NavigationKeyBinding::default_next_group())
    }
    pub fn previous_group(&self) -> KeyDefinition {
        self.previous_group
            .unwrap_or(NavigationKeyBinding::default_previous_group())
    }
}
impl NavigationKeyBinding {
    fn default_left() -> KeyDefinition {
        KeyDefinition {
            key: Some('h'),
            modifier: None,
        }
    }
    fn default_right() -> KeyDefinition {
        KeyDefinition {
            key: Some('l'),
            modifier: None,
        }
    }
    fn default_up() -> KeyDefinition {
        KeyDefinition {
            key: Some('k'),
            modifier: None,
        }
    }
    fn default_down() -> KeyDefinition {
        KeyDefinition {
            key: Some('j'),
            modifier: None,
        }
    }
    fn default_next_group() -> KeyDefinition {
        KeyDefinition {
            key: Some('g'),
            modifier: None,
        }
    }
    fn default_previous_group() -> KeyDefinition {
        KeyDefinition {
            key: Some('G'),
            modifier: Some(KeyModifier::Shift),
        }
    }
}
