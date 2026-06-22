use zellij_tile::prelude::*;

pub enum InputAction {
    MoveUp,
    MoveDown,
    CycleMode,
    Confirm,
    DeleteChar,
    TogglePreview,
    Cancel,
    TypeChar(char),
}

pub fn map_key(key: &KeyWithModifier) -> Option<InputAction> {
    let has_ctrl = key.key_modifiers.contains(&KeyModifier::Ctrl);
    match key.bare_key {
        BareKey::Up => Some(InputAction::MoveUp),
        BareKey::Down => Some(InputAction::MoveDown),
        BareKey::Tab => Some(InputAction::CycleMode),
        BareKey::Enter => Some(InputAction::Confirm),
        BareKey::Backspace => Some(InputAction::DeleteChar),
        BareKey::Char('c') if has_ctrl => Some(InputAction::Cancel),
        BareKey::Char('e') if has_ctrl => Some(InputAction::TogglePreview),
        BareKey::Char(c) if !has_ctrl => Some(InputAction::TypeChar(c)),
        BareKey::Esc => Some(InputAction::Cancel),
        _ => None,
    }
}
