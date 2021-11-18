use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum KeyModifier {
    Shift,
    Control,
    Alt,
}

impl KeyModifier {
    pub fn crossterm(modifier: Option<Self>) -> KeyModifiers {
        match modifier {
            Some(m) => match m {
                KeyModifier::Shift => KeyModifiers::SHIFT,
                KeyModifier::Alt => KeyModifiers::ALT,
                KeyModifier::Control => KeyModifiers::CONTROL,
            },
            None => KeyModifiers::NONE,
        }
    }
}

#[derive(Merge, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct KeyDefinition {
    pub key: Option<char>,
    pub modifier: Option<KeyModifier>,
}

impl KeyDefinition {
    pub fn crossterm(&self) -> Event {
        Event::Key(KeyEvent {
            code: KeyCode::Char(self.key.unwrap()),
            modifiers: KeyModifier::crossterm(self.modifier),
        })
    }
}
