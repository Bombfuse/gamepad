#[cfg(not(any(target_family = "wasm", target_os = "android")))]
pub(crate) mod gilrs;

#[cfg(any(target_family = "wasm", target_os = "android"))]
pub(crate) mod dummy;

use crate::types::*;

pub(crate) trait GamepadEngineBackend {
    /// This should be called every frame.
    fn update(&mut self) -> Result<(), GamepadError> {
        Ok(())
    }

    /// This should be called to retrieve all of the events since the last update.
    fn poll_events(&mut self) -> Vec<GamepadEvent> {
        Vec::new()
    }

    fn gamepads(&self) -> &Vec<GamepadState>;
    fn gamepads_mut(&mut self) -> &mut Vec<GamepadState>;
}
