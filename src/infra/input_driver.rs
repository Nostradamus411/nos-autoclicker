use enigo::{Enigo, Mouse, Button, Coordinate, Settings};
use crate::domain::settings::ClickAction;

/// Perform a single mouse click of the given action type.
pub fn perform_click(action: ClickAction) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| format!("Enigo init error: {e}"))?;

    match action {
        ClickAction::Left => {
            enigo
                .button(Button::Left, enigo::Direction::Click)
                .map_err(|e| format!("Left click failed: {e}"))?;
        }
        ClickAction::Right => {
            enigo
                .button(Button::Right, enigo::Direction::Click)
                .map_err(|e| format!("Right click failed: {e}"))?;
        }
        ClickAction::Middle => {
            enigo
                .button(Button::Middle, enigo::Direction::Click)
                .map_err(|e| format!("Middle click failed: {e}"))?;
        }
        ClickAction::Double => {
            enigo
                .button(Button::Left, enigo::Direction::Click)
                .map_err(|e| format!("Double click (1) failed: {e}"))?;
            std::thread::sleep(std::time::Duration::from_millis(30));
            enigo
                .button(Button::Left, enigo::Direction::Click)
                .map_err(|e| format!("Double click (2) failed: {e}"))?;
        }
    }

    Ok(())
}

/// Move the mouse to a fixed position before clicking.
pub fn move_to(x: i32, y: i32) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| format!("Enigo init error: {e}"))?;
    enigo
        .move_mouse(x, y, Coordinate::Abs)
        .map_err(|e| format!("Move mouse failed: {e}"))?;
    Ok(())
}

/// Get the current mouse position using device_query.
pub fn get_mouse_position() -> (i32, i32) {
    use device_query::{DeviceQuery, DeviceState};
    let device_state = DeviceState::new();
    let pos = device_state.get_mouse().coords;
    (pos.0, pos.1)
}

/// Check if a specific window is focused (stub for cross-platform).
/// On Linux/WSL2 dev: always returns true. 
/// On Windows runtime: would use Win32 GetForegroundWindow.
pub fn is_target_window_focused() -> bool {
    // In development (Linux/WSL2), always treat as focused.
    // Windows-specific focus detection will be added for release builds.
    #[cfg(target_os = "windows")]
    {
        // TODO: Implement Win32 GetForegroundWindow check for Windows runtime
        true
    }
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}
