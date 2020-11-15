use crate::backends::GamepadEngineBackend;
use crate::types::*;

use winapi::{
    shared::{
        minwindef::DWORD,
        winerror::{ERROR_DEVICE_NOT_CONNECTED, ERROR_SUCCESS},
    },
    um::xinput::*,
};

pub(crate) struct XInputBackend {
    gamepads: Vec<GamepadState>,
}
impl XInputBackend {
    pub fn new() -> Self {
        XInputBackend {
            gamepads: Vec::new(),
        }
    }

    fn get_input_state(&mut self, i: DWORD) -> Result<XInputState, GamepadError> {
        let mut output: XINPUT_STATE = unsafe { ::std::mem::zeroed() };
        let result: DWORD = unsafe { XInputGetState(i, &mut output) };

        match result {
            ERROR_SUCCESS => Ok(XInputState { raw: output }),
            ERROR_DEVICE_NOT_CONNECTED => Err(GamepadError::new(
                format!("Device not connected for slot {}", i),
                ErrorType::GamepadNotConnected { slot: i as u8 },
            )),
            _ => Err(GamepadError::new(
                format!("Error code: {}", result),
                ErrorType::Unknown,
            )),
        }
    }
}

impl GamepadEngineBackend for XInputBackend {
    fn update(&mut self) -> Result<(), GamepadError> {
        let mut gamepads = Vec::new();

        for i in 0..XUSER_MAX_COUNT {
            match self.get_input_state(i) {
                Ok(state) => gamepads.push(state.to_gamepad()),
                Err(e) => {
                    match e.error_type {
                        ErrorType::GamepadNotConnected { slot: _ } => {} // We can ignore these for now, this will happen every frame where there is an empty gamepad slot
                        ErrorType::Unknown => return Err(e),
                    }
                }
            }
        }

        // Enter the previous gamepads' states for the new gamepads
        for i in 0..self.gamepads.len() {
            if let Some(prev_gamepad) = self.gamepads.get(i) {
                if let Some(new_gamepad) = gamepads.get_mut(i) {
                    for (key, button_state) in prev_gamepad.buttons() {
                        let state = new_gamepad
                            .buttons
                            .entry(key.clone())
                            .or_insert(ButtonState::default());
                        state.was_pressed = button_state.is_pressed;
                    }
                }
            }
        }

        self.gamepads = gamepads;

        Ok(())
    }

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

/// Source code https://github.com/Lokathor/rusty-xinput/blob/master/src/lib.rs
/// This code is altered slightly from the original.
///
/// This wraps an `XINPUT_STATE` value and provides a more rusty (read-only)
/// interface to the data it contains.
///
/// All three major game companies use different names for most of the buttons,
/// so the docs for each button method list out what each of the major companies
/// call that button. To the driver it's all the same, it's just however you
/// want to think of them.
///
/// If sequential calls to `xinput_get_state` for a given controller slot have
/// the same packet number then the controller state has not changed since the
/// last call. The `PartialEq` and `Eq` implementations for this wrapper type
/// reflect that. The exact value of the packet number is unimportant.
///
/// If you want to do something that the rust wrapper doesn't support, just use
/// the raw field to get at the inner value.
pub(crate) struct XInputState {
    /// The raw value we're wrapping.
    pub raw: XINPUT_STATE,
}

impl ::std::cmp::PartialEq for XInputState {
    /// Equality for `XInputState` values is based _only_ on the
    /// `dwPacketNumber` of the wrapped `XINPUT_STATE` value. This is entirely
    /// correct for values obtained from the xinput system, but if you make your
    /// own `XInputState` values for some reason you can confuse it.
    fn eq(&self, other: &XInputState) -> bool {
        self.raw.dwPacketNumber == other.raw.dwPacketNumber
    }
}

impl ::std::cmp::Eq for XInputState {}

impl ::std::fmt::Debug for XInputState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "XInputState (_)")
    }
}

impl XInputState {
    pub fn to_gamepad(self) -> GamepadState {
        let mut gamepad = GamepadState::new();
        let buttons = &mut gamepad.buttons;
        buttons.insert(Button::DPadNorth, ButtonState::new(self.arrow_up(), false));
        buttons.insert(
            Button::DPadSouth,
            ButtonState::new(self.arrow_down(), false),
        );
        buttons.insert(
            Button::DPadEast,
            ButtonState::new(self.arrow_right(), false),
        );
        buttons.insert(Button::DPadWest, ButtonState::new(self.arrow_left(), false));

        buttons.insert(Button::North, ButtonState::new(self.north_button(), false));
        buttons.insert(Button::South, ButtonState::new(self.south_button(), false));
        buttons.insert(Button::East, ButtonState::new(self.east_button(), false));
        buttons.insert(Button::West, ButtonState::new(self.west_button(), false));

        buttons.insert(
            Button::LeftShoulder,
            ButtonState::new(self.left_shoulder(), false),
        );
        buttons.insert(
            Button::LeftTrigger,
            ButtonState::new(self.left_trigger_bool(), false),
        );

        buttons.insert(
            Button::RightShoulder,
            ButtonState::new(self.right_shoulder(), false),
        );
        buttons.insert(
            Button::RightTrigger,
            ButtonState::new(self.right_trigger_bool(), false),
        );

        buttons.insert(
            Button::RightStick,
            ButtonState::new(self.right_thumb_button(), false),
        );
        buttons.insert(
            Button::LeftStick,
            ButtonState::new(self.left_thumb_button(), false),
        );

        buttons.insert(
            Button::Select,
            ButtonState::new(self.select_button(), false),
        );
        buttons.insert(Button::Start, ButtonState::new(self.start_button(), false));
        buttons.insert(Button::Menu, ButtonState::new(false, false));

        gamepad
    }

    /// The north button of the action button group.
    ///
    /// * Nintendo: X
    /// * Playstation: Triangle
    /// * XBox: Y
    #[inline]
    pub fn north_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_Y != 0
    }

    /// The south button of the action button group.
    ///
    /// * Nintendo: B
    /// * Playstation: X
    /// * XBox: A
    #[inline]
    pub fn south_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_A != 0
    }

    /// The east button of the action button group.
    ///
    /// * Nintendo: A
    /// * Playstation: Circle
    /// * XBox: B
    #[inline]
    pub fn east_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_B != 0
    }

    /// The west button of the action button group.
    ///
    /// * Nintendo: Y
    /// * Playstation: Square
    /// * XBox: X
    #[inline]
    pub fn west_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_X != 0
    }

    /// The up button on the directional pad.
    #[inline]
    pub fn arrow_up(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_UP != 0
    }

    /// The down button on the directional pad.
    #[inline]
    pub fn arrow_down(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_DOWN != 0
    }

    /// The left button on the directional pad.
    #[inline]
    pub fn arrow_left(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_LEFT != 0
    }

    /// The right button on the directional pad.
    #[inline]
    pub fn arrow_right(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_RIGHT != 0
    }

    /// The "start" button.
    ///
    /// * Nintendo: Start (NES / SNES), '+' (Pro Controller)
    /// * Playstation: Start
    /// * XBox: Start
    #[inline]
    pub fn start_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_START != 0
    }

    /// The "not start" button.
    ///
    /// * Nintendo: Select (NES / NES), '-' (Pro Controller)
    /// * Playstation: Select
    /// * XBox: Back
    #[inline]
    pub fn select_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_BACK != 0
    }

    /// The upper left shoulder button.
    ///
    /// * Nintendo: L
    /// * Playstation: L1
    /// * XBox: LB
    #[inline]
    pub fn left_shoulder(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_SHOULDER != 0
    }

    /// The upper right shoulder button.
    ///
    /// * Nintendo: R
    /// * Playstation: R1
    /// * XBox: RB
    #[inline]
    pub fn right_shoulder(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_SHOULDER != 0
    }

    /// The default threshold to count a trigger as being "pressed".
    pub const TRIGGER_THRESHOLD: u8 = XINPUT_GAMEPAD_TRIGGER_THRESHOLD;

    /// The lower left shoulder trigger. If you want to use this as a simple
    /// boolean it is suggested that you compare it to the `TRIGGER_THRESHOLD`
    /// constant.
    ///
    /// * Nintendo: ZL
    /// * Playstation: L2
    /// * XBox: LT
    #[inline]
    pub fn left_trigger(&self) -> u8 {
        self.raw.Gamepad.bLeftTrigger
    }

    /// The lower right shoulder trigger. If you want to use this as a simple
    /// boolean it is suggested that you compare it to the `TRIGGER_THRESHOLD`
    /// constant.
    ///
    /// * Nintendo: ZR
    /// * Playstation: R2
    /// * XBox: RT
    #[inline]
    pub fn right_trigger(&self) -> u8 {
        self.raw.Gamepad.bRightTrigger
    }

    /// The lower left shoulder trigger as a bool using the default threshold.
    ///
    /// * Nintendo: ZL
    /// * Playstation: L2
    /// * XBox: LT
    #[inline]
    pub fn left_trigger_bool(&self) -> bool {
        self.left_trigger() >= XInputState::TRIGGER_THRESHOLD
    }

    /// The lower right shoulder trigger as a bool using the default threshold.
    ///
    /// * Nintendo: ZR
    /// * Playstation: R2
    /// * XBox: RT
    #[inline]
    pub fn right_trigger_bool(&self) -> bool {
        self.right_trigger() >= XInputState::TRIGGER_THRESHOLD
    }

    /// The left thumb stick being pressed inward.
    ///
    /// * Nintendo: (L)
    /// * Playstation: L3
    /// * XBox: (L)
    #[inline]
    pub fn left_thumb_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_THUMB != 0
    }

    /// The right thumb stick being pressed inward.
    ///
    /// * Nintendo: (R)
    /// * Playstation: R3
    /// * XBox: (R)
    #[inline]
    pub fn right_thumb_button(&self) -> bool {
        self.raw.Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_THUMB != 0
    }

    /// The suggested default deadzone for use with the left thumb stick.
    pub const LEFT_STICK_DEADZONE: i16 = XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE;

    /// The suggested default deadzone for use with the right thumb stick.
    pub const RIGHT_STICK_DEADZONE: i16 = XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE;

    /// The left stick raw value.
    ///
    /// Positive values are to the right (X-axis) or up (Y-axis).
    #[inline]
    pub fn left_stick_raw(&self) -> (i16, i16) {
        (self.raw.Gamepad.sThumbLX, self.raw.Gamepad.sThumbLY)
    }

    /// The right stick raw value.
    ///
    /// Positive values are to the right (X-axis) or up (Y-axis).
    #[inline]
    pub fn right_stick_raw(&self) -> (i16, i16) {
        (self.raw.Gamepad.sThumbRX, self.raw.Gamepad.sThumbRY)
    }

    /// The left stick value normalized with the default dead-zone.
    ///
    /// See `normalize_raw_stick_value` for more.
    #[inline]
    pub fn left_stick_normalized(&self) -> (f32, f32) {
        XInputState::normalize_raw_stick_value(
            self.left_stick_raw(),
            XInputState::LEFT_STICK_DEADZONE,
        )
    }

    /// The right stick value normalized with the default dead-zone.
    ///
    /// See `normalize_raw_stick_value` for more.
    #[inline]
    pub fn right_stick_normalized(&self) -> (f32, f32) {
        XInputState::normalize_raw_stick_value(
            self.right_stick_raw(),
            XInputState::RIGHT_STICK_DEADZONE,
        )
    }

    /// This helper normalizes a raw stick value using the given deadzone.
    ///
    /// If the raw value's 2d length is less than the deadzone the result will be
    /// `(0.0,0.0)`, otherwise the result is normalized across the range from the
    /// deadzone point to the maximum value.
    ///
    /// The `deadzone` value is clamped to the range 0 to 32,766 (inclusive)
    /// before use. Negative inputs or maximum value inputs make the normalization
    /// just work improperly.
    #[inline]
    pub fn normalize_raw_stick_value(raw_stick: (i16, i16), deadzone: i16) -> (f32, f32) {
        let deadzone_float = deadzone.max(0).min(i16::max_value() - 1) as f32;
        let raw_float = (raw_stick.0 as f32, raw_stick.1 as f32);
        let length = (raw_float.0 * raw_float.0 + raw_float.1 * raw_float.1).sqrt();
        let normalized = (raw_float.0 / length, raw_float.1 / length);
        if length > deadzone_float {
            // clip our value to the expected maximum length.
            let length = length.min(32_767.0);
            let scale = (length - deadzone_float) / (32_767.0 - deadzone_float);
            (normalized.0 * scale, normalized.1 * scale)
        } else {
            (0.0, 0.0)
        }
    }
}
