
use crate::backends::{GamepadEngineBackend};

extern "C" {
    fn initiate_gamepad_update();
}
  
#[no_mangle]
extern "C" fn update_gamepads() {
}


pub struct WasmBackend {
    gamepads: Vec<GamepadState>,
}
impl WasmBackend {
    pub fn new() -> Self {
        WasmBackend {
            gamepads: Vec::new(),
        }
    }
}
impl GamepadEngineBackend for WasmBackend {
    fn gamepads(&self) -> &Vec<GamepadState> { &self.gamepads }
    fn gamepads_mut(&mut self) -> &mut Vec<GamepadState> { &mut self.gamepads }
}

/// https://developer.mozilla.org/en-US/docs/Games/Techniques/Controls_Gamepad_API
/// 
/// The buttons array contains the Xbox 360 button layout:
/// buttons: [
///   'DPad-Up','DPad-Down','DPad-Left','DPad-Right',
///   'Start','Back','Axis-Left','Axis-Right',
///   'LB','RB','Power','A','B','X','Y',
/// ],
const DPAD_UP_INDEX: usize = 0;
const DPAD_DOWN_INDEX: usize = 1;
const DPAD_LEFT_INDEX: usize = 2;
const DPAD_RIGHT_INDEX: usize = 3;
const START_INDEX: usize = 4;
const BACK_INDEX: usize = 5;
const AXIS_LEFT_INDEX: usize = 6;
const AXIS_RIGHT_INDEX: usize = 7;
const LEFT_SHOULDER_INDEX: usize = 8;
const RIGHT_SHOULDER_INDEX: usize = 9;
const BUTTON_SOUTH_INDEX: usize = 10;
const BUTTON_EAST_INDEX: usize = 11;
const BUTTON_WEST_INDEX: usize = 12;
const BUTTON_NORTH_INDEX: usize = 13;

