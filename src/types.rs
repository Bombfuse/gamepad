use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ErrorType {
    GamepadNotConnected { slot: u8 },
    Unknown, // Uncommon errors not documented by gamepad lib
}

#[derive(Debug, Clone)]
pub struct GamepadError {
    pub msg: String,
    pub error_type: ErrorType,
}
impl GamepadError {
    pub fn new<T: Into<String>>(msg: T, error_type: ErrorType) -> Self {
        GamepadError {
            msg: msg.into(),
            error_type,
        }
    }
}

#[derive(Clone, Debug)]
pub enum GamepadEvent {
    Connected {},
    Disconnected {},
}

#[derive(Clone, Debug)]
pub struct GamepadState {
    pub(crate) buttons: HashMap<Button, ButtonState>,
}
impl GamepadState {
    pub fn new() -> Self {
        GamepadState {
            buttons: HashMap::new(),
        }
    }

    pub fn buttons(&self) -> &HashMap<Button, ButtonState> {
        &self.buttons
    }

    pub fn buttons_mut(&mut self) -> &mut HashMap<Button, ButtonState> {
        &mut self.buttons
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        match self.buttons.get(&button) {
            Some(button_state) => button_state.is_pressed,
            None => false,
        }
    }

    pub fn is_just_pressed(&self, button: Button) -> bool {
        match self.buttons.get(&button) {
            Some(button_state) => button_state.is_pressed && !button_state.was_pressed,
            None => false,
        }
    }

    pub fn is_just_released(&self, button: Button) -> bool {
        match self.buttons.get(&button) {
            Some(button_state) => !button_state.is_pressed && button_state.was_pressed,
            None => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ButtonState {
    pub(crate) is_pressed: bool,
    pub(crate) was_pressed: bool,
}
impl ButtonState {
    pub fn new(is_pressed: bool, was_pressed: bool) -> Self {
        ButtonState {
            is_pressed,
            was_pressed,
        }
    }
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn is_just_pressed(&self) -> bool {
        self.is_pressed && !self.was_pressed
    }

    pub fn is_just_released(&self) -> bool {
        !self.is_pressed && self.was_pressed
    }
}
impl Default for ButtonState {
    fn default() -> ButtonState {
        ButtonState::new(false, false)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Button {
    DPadNorth,
    DPadSouth,
    DPadWest,
    DPadEast,
    North,
    South,
    West,
    East,
    LeftShoulder,
    RightShoulder,
    LeftTrigger,
    RightTrigger,
    RightStick,
    LeftStick,
    Menu,
    Select,
    Start,
}
