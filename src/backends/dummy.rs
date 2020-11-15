use crate::backends::GamepadEngineBackend;
use crate::types::GamepadState;

pub(crate) struct DummyBackend {
    gamepads: Vec<GamepadState>,
}
impl DummyBackend {
    pub fn new() -> Self {
        DummyBackend {
            gamepads: Vec::new(),
        }
    }
}
impl GamepadEngineBackend for DummyBackend {
    fn gamepads(&self) -> &Vec<GamepadState> {
        &self.gamepads
    }
    fn gamepads_mut(&mut self) -> &mut Vec<GamepadState> {
        &mut self.gamepads
    }
}
