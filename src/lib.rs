mod backends;
mod types;

pub use types::*;

#[cfg(target_os = "windows")]
use backends::xinput::XInputBackend as Backend;

#[cfg(not(target_os = "windows"))]
use backends::dummy::DummyBackend as Backend;

pub struct GamepadEngine {
    backend: Box<dyn crate::backends::GamepadEngineBackend>,
}
impl GamepadEngine {
    /// Instantiates gamepad engine, begins polling for input
    pub fn new() -> Self {
        GamepadEngine {
            backend: Box::new(Backend::new()),
        }
    }

    /// Polls for input and updates all gamepad states
    pub fn update(&mut self) -> Result<(), GamepadError> {
        self.backend.update()?;

        Ok(())
    }

    pub fn gamepads(&self) -> &Vec<GamepadState> {
        self.backend.gamepads()
    }

    pub fn gamepads_mut(&mut self) -> &mut Vec<GamepadState> {
        self.backend.gamepads_mut()
    }
}
