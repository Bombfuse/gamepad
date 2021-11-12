use crate::backends::GamepadEngineBackend;
use crate::types::*;

use gilrs::{Axis, Button as GilrsButton, Gamepad, Gilrs};

fn get_gilrs_to_gamepad_buttons() -> Vec<(GilrsButton, Button)> {
    vec![
        (GilrsButton::South, Button::South),
        (GilrsButton::North, Button::North),
        (GilrsButton::East, Button::East),
        (GilrsButton::West, Button::West),
        (GilrsButton::DPadUp, Button::DPadNorth),
        (GilrsButton::DPadDown, Button::DPadSouth),
        (GilrsButton::DPadLeft, Button::DPadWest),
        (GilrsButton::DPadRight, Button::DPadEast),
        (GilrsButton::Start, Button::Start),
        (GilrsButton::Select, Button::Select),
        (GilrsButton::LeftTrigger, Button::LeftTrigger),
        (GilrsButton::LeftTrigger2, Button::LeftShoulder),
        (GilrsButton::RightTrigger, Button::RightTrigger),
        (GilrsButton::RightTrigger2, Button::RightShoulder),
    ]
}

pub struct GilrsBackend {
    gilrs: Gilrs,
    gamepads: Vec<GamepadState>,
}
impl GilrsBackend {
    pub fn new() -> Self {
        GilrsBackend {
            gilrs: Gilrs::new().unwrap(),
            gamepads: Vec::new(),
        }
    }
}
impl GamepadEngineBackend for GilrsBackend {
    /// This should be called every frame.
    fn update(&mut self) -> Result<(), GamepadError> {
        while let Some(_) = self.gilrs.next_event() {}

        let gamepads = self
            .gilrs
            .gamepads()
            .map(|(_gamepad_id, gamepad)| gamepad.clone())
            .collect::<Vec<Gamepad>>();
        let mut new_gamepads = Vec::new();
        let mut i = 0;
        for gamepad in gamepads {
            let mut gamepad_state = GamepadState::new();

            for (gilrs_button, button) in get_gilrs_to_gamepad_buttons() {
                let is_pressed = gamepad.is_pressed(gilrs_button);
                let mut was_pressed = false;

                if let Some(prev_gamepad) = self.gamepads.get(i) {
                    was_pressed = prev_gamepad.is_pressed(button.clone());
                }

                gamepad_state
                    .buttons
                    .insert(button, ButtonState::new(is_pressed, was_pressed));
            }

            if let (Some(left_x), Some(left_y)) = (
                gamepad.axis_data(Axis::LeftStickX),
                gamepad.axis_data(Axis::LeftStickY),
            ) {
                gamepad_state.joysticks.insert(
                    Joystick::Left,
                    JoystickState::new(
                        (left_x.value() as i16, left_y.value() as i16),
                        (left_x.value(), left_y.value()),
                    ),
                );
            }

            if let (Some(right_x), Some(right_y)) = (
                gamepad.axis_data(Axis::RightStickX),
                gamepad.axis_data(Axis::RightStickY),
            ) {
                gamepad_state.joysticks.insert(
                    Joystick::Right,
                    JoystickState::new(
                        (right_x.value() as i16, right_y.value() as i16),
                        (right_x.value(), right_y.value()),
                    ),
                );
            }

            new_gamepads.push(gamepad_state);
            i += 1;
        }

        self.gamepads = new_gamepads;

        Ok(())
    }

    /// This should be called to retrieve all of the events since the last update.
    fn poll_events(&mut self) -> Vec<GamepadEvent> {
        Vec::new()
    }

    fn gamepads(&self) -> &Vec<GamepadState> {
        &self.gamepads
    }

    fn gamepads_mut(&mut self) -> &mut Vec<GamepadState> {
        &mut self.gamepads
    }
}
