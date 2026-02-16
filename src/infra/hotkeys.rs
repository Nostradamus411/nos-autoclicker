use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

/// Hotkey action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    Start,
    Stop,
    PanicStop,
}

/// Manages global hotkey registration and event dispatch.
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    start_hotkey: Option<HotKey>,
    stop_hotkey: Option<HotKey>,
    panic_hotkey: Option<HotKey>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self, String> {
        let manager =
            GlobalHotKeyManager::new().map_err(|e| format!("Failed to create hotkey manager: {e}"))?;
        Ok(Self {
            manager,
            start_hotkey: None,
            stop_hotkey: None,
            panic_hotkey: None,
        })
    }

    /// Parse a hotkey string like "F6" or "Ctrl+Shift+S" into a HotKey.
    pub fn parse_hotkey(key_str: &str) -> Result<HotKey, String> {
        let parts: Vec<&str> = key_str.split('+').map(|s| s.trim()).collect();
        let mut modifiers = Modifiers::empty();
        let mut code = None;

        for part in &parts {
            match part.to_lowercase().as_str() {
                "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
                "shift" => modifiers |= Modifiers::SHIFT,
                "alt" => modifiers |= Modifiers::ALT,
                "super" | "meta" | "win" => modifiers |= Modifiers::SUPER,
                key => {
                    code = Some(string_to_code(key)?);
                }
            }
        }

        let code = code.ok_or_else(|| format!("No key code found in '{key_str}'"))?;

        if modifiers.is_empty() {
            Ok(HotKey::new(None, code))
        } else {
            Ok(HotKey::new(Some(modifiers), code))
        }
    }

    /// Register start, stop, and panic hotkeys.
    pub fn register(
        &mut self,
        start_key: &str,
        stop_key: &str,
        panic_key: &str,
    ) -> Result<(), String> {
        let start = Self::parse_hotkey(start_key)?;
        let stop = Self::parse_hotkey(stop_key)?;
        let panic = Self::parse_hotkey(panic_key)?;

        self.manager
            .register(start)
            .map_err(|e| format!("Failed to register start hotkey: {e}"))?;
        self.manager
            .register(stop)
            .map_err(|e| format!("Failed to register stop hotkey: {e}"))?;
        self.manager
            .register(panic)
            .map_err(|e| format!("Failed to register panic hotkey: {e}"))?;

        self.start_hotkey = Some(start);
        self.stop_hotkey = Some(stop);
        self.panic_hotkey = Some(panic);

        Ok(())
    }

    /// Unregister all hotkeys.
    pub fn unregister_all(&mut self) {
        if let Some(hk) = self.start_hotkey.take() {
            let _ = self.manager.unregister(hk);
        }
        if let Some(hk) = self.stop_hotkey.take() {
            let _ = self.manager.unregister(hk);
        }
        if let Some(hk) = self.panic_hotkey.take() {
            let _ = self.manager.unregister(hk);
        }
    }

    /// Check for a pending hotkey event and return the matching action.
    pub fn poll_event(&self) -> Option<HotkeyAction> {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if let Some(ref hk) = self.start_hotkey {
                if event.id() == hk.id() {
                    return Some(HotkeyAction::Start);
                }
            }
            if let Some(ref hk) = self.stop_hotkey {
                if event.id() == hk.id() {
                    return Some(HotkeyAction::Stop);
                }
            }
            if let Some(ref hk) = self.panic_hotkey {
                if event.id() == hk.id() {
                    return Some(HotkeyAction::PanicStop);
                }
            }
        }
        None
    }
}

fn string_to_code(key: &str) -> Result<Code, String> {
    match key.to_lowercase().as_str() {
        "f1" => Ok(Code::F1),
        "f2" => Ok(Code::F2),
        "f3" => Ok(Code::F3),
        "f4" => Ok(Code::F4),
        "f5" => Ok(Code::F5),
        "f6" => Ok(Code::F6),
        "f7" => Ok(Code::F7),
        "f8" => Ok(Code::F8),
        "f9" => Ok(Code::F9),
        "f10" => Ok(Code::F10),
        "f11" => Ok(Code::F11),
        "f12" => Ok(Code::F12),
        "a" => Ok(Code::KeyA),
        "b" => Ok(Code::KeyB),
        "c" => Ok(Code::KeyC),
        "d" => Ok(Code::KeyD),
        "e" => Ok(Code::KeyE),
        "f" => Ok(Code::KeyF),
        "g" => Ok(Code::KeyG),
        "h" => Ok(Code::KeyH),
        "i" => Ok(Code::KeyI),
        "j" => Ok(Code::KeyJ),
        "k" => Ok(Code::KeyK),
        "l" => Ok(Code::KeyL),
        "m" => Ok(Code::KeyM),
        "n" => Ok(Code::KeyN),
        "o" => Ok(Code::KeyO),
        "p" => Ok(Code::KeyP),
        "q" => Ok(Code::KeyQ),
        "r" => Ok(Code::KeyR),
        "s" => Ok(Code::KeyS),
        "t" => Ok(Code::KeyT),
        "u" => Ok(Code::KeyU),
        "v" => Ok(Code::KeyV),
        "w" => Ok(Code::KeyW),
        "x" => Ok(Code::KeyX),
        "y" => Ok(Code::KeyY),
        "z" => Ok(Code::KeyZ),
        "space" => Ok(Code::Space),
        "escape" | "esc" => Ok(Code::Escape),
        "enter" | "return" => Ok(Code::Enter),
        "tab" => Ok(Code::Tab),
        "backspace" => Ok(Code::Backspace),
        "delete" | "del" => Ok(Code::Delete),
        "insert" | "ins" => Ok(Code::Insert),
        "home" => Ok(Code::Home),
        "end" => Ok(Code::End),
        "pageup" => Ok(Code::PageUp),
        "pagedown" => Ok(Code::PageDown),
        "up" => Ok(Code::ArrowUp),
        "down" => Ok(Code::ArrowDown),
        "left" => Ok(Code::ArrowLeft),
        "right" => Ok(Code::ArrowRight),
        _ => Err(format!("Unknown key: '{key}'")),
    }
}
