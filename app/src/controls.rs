use chargrid::input::{GamepadButton, KeyboardInput};
use direction::CardinalDirection;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub enum AppInput {
    Move(CardinalDirection),
    Tech,
    Wait,
    Ability(u8),
    Examine,
}

#[derive(Serialize, Deserialize)]
pub struct Controls {
    keys: HashMap<KeyboardInput, AppInput>,
    gamepad: HashMap<GamepadButton, AppInput>,
}

impl Controls {
    pub fn default() -> Self {
        let keys = hashmap![
            KeyboardInput::Left => AppInput::Move(CardinalDirection::West),
            KeyboardInput::Right => AppInput::Move(CardinalDirection::East),
            KeyboardInput::Up => AppInput::Move(CardinalDirection::North),
            KeyboardInput::Down => AppInput::Move(CardinalDirection::South),
            KeyboardInput::Char('a') => AppInput::Move(CardinalDirection::West),
            KeyboardInput::Char('d') => AppInput::Move(CardinalDirection::East),
            KeyboardInput::Char('w') => AppInput::Move(CardinalDirection::North),
            KeyboardInput::Char('s') => AppInput::Move(CardinalDirection::South),
            KeyboardInput::Char('h') => AppInput::Move(CardinalDirection::West),
            KeyboardInput::Char('l') => AppInput::Move(CardinalDirection::East),
            KeyboardInput::Char('k') => AppInput::Move(CardinalDirection::North),
            KeyboardInput::Char('j') => AppInput::Move(CardinalDirection::South),
            KeyboardInput::Char('t') => AppInput::Tech,
            KeyboardInput::Char('x') => AppInput::Examine,
            KeyboardInput::Char(' ') => AppInput::Wait,
            KeyboardInput::Char('1') => AppInput::Ability(0),
            KeyboardInput::Char('2') => AppInput::Ability(1),
            KeyboardInput::Char('3') => AppInput::Ability(2),
            KeyboardInput::Char('4') => AppInput::Ability(3),
            KeyboardInput::Char('5') => AppInput::Ability(4),
            KeyboardInput::Char('6') => AppInput::Ability(5),
            KeyboardInput::Char('7') => AppInput::Ability(6),
            KeyboardInput::Char('8') => AppInput::Ability(7),
        ];
        let gamepad = hashmap![
            GamepadButton::DPadLeft => AppInput::Move(CardinalDirection::West),
            GamepadButton::DPadRight => AppInput::Move(CardinalDirection::East),
            GamepadButton::DPadUp => AppInput::Move(CardinalDirection::North),
            GamepadButton::DPadDown => AppInput::Move(CardinalDirection::South),
        ];
        Self { keys, gamepad }
    }

    pub fn get(&self, keyboard_input: KeyboardInput) -> Option<AppInput> {
        self.keys.get(&keyboard_input).cloned()
    }

    pub fn get_gamepad(&self, gamepad_input: GamepadButton) -> Option<AppInput> {
        self.gamepad.get(&gamepad_input).cloned()
    }
}
