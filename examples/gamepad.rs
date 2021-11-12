use gamepad::*;

use std::time::Duration;

pub fn main() {
    let mut engine = GamepadEngine::new();

    loop {
        engine.update().unwrap();

        for gamepad in engine.gamepads() {
            for (key, button) in gamepad.buttons() {
                if button.is_just_pressed() {
                    println!("Just Pressed: {:?}", key);
                }
            }

            for (_, joystick) in gamepad.joysticks() {
                println!("{:?}", joystick);
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}
