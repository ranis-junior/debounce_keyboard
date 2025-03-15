pub mod debounce {
    use crate::device::windows::config::ConfigHolder;
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};
    use strum::EnumIter;

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
    #[allow(non_camel_case_types)]
    pub enum KeybdKey {
        KEY_ESC,
        KEY_1,
        KEY_2,
        KEY_3,
        KEY_4,
        KEY_5,
        KEY_6,
        KEY_7,
        KEY_8,
        KEY_9,
        KEY_0,
        KEY_MINUS,
        KEY_EQUAL,
        KEY_BACKSPACE,
        KEY_TAB,
        KEY_Q,
        KEY_W,
        KEY_E,
        KEY_R,
        KEY_T,
        KEY_Y,
        KEY_U,
        KEY_I,
        KEY_O,
        KEY_P,
        KEY_LEFTBRACE,
        KEY_RIGHTBRACE,
        KEY_ENTER,
        KEY_LEFTCTRL,
        KEY_A,
        KEY_S,
        KEY_D,
        KEY_F,
        KEY_G,
        KEY_H,
        KEY_J,
        KEY_K,
        KEY_L,
        KEY_SEMICOLON,
        KEY_APOSTROPHE,
        KEY_GRAVE,
        KEY_LEFTSHIFT,
        KEY_BACKSLASH,
        KEY_Z,
        KEY_X,
        KEY_C,
        KEY_V,
        KEY_B,
        KEY_N,
        KEY_M,
        KEY_COMMA,
        KEY_DOT,
        KEY_SLASH,
        KEY_RIGHTSHIFT,
        KEY_LEFTALT,
        KEY_SPACE,
        KEY_CAPSLOCK,
        KEY_F1,
        KEY_F2,
        KEY_F3,
        KEY_F4,
        KEY_F5,
        KEY_F6,
        KEY_F7,
        KEY_F8,
        KEY_F9,
        KEY_F10,
        KEY_NUMLOCK,
        KEY_SCROLLLOCK,
        KEY_KP7,
        KEY_KP8,
        KEY_KP9,
        KEY_KPMINUS,
        KEY_KP4,
        KEY_KP5,
        KEY_KP6,
        KEY_KPPLUS,
        KEY_KP1,
        KEY_KP2,
        KEY_KP3,
        KEY_KP0,
        KEY_KPDOT,
        KEY_F11,
        KEY_F12,
        KEY_HOME,
        KEY_UP,
        KEY_PAGEUP,
        KEY_LEFT,
        KEY_RIGHT,
        KEY_END,
        KEY_DOWN,
        KEY_PAGEDOWN,
        KEY_INSERT,
        KEY_DELETE,
        KEY_MUTE,
        KEY_VOLUMEDOWN,
        KEY_VOLUMEUP,
        KEY_PLAYPAUSE,
        KEY_STOPCD,
        KEY_NEXTSONG,
        KEY_PREVIOUSSONG,
        KEY_RIGHTCTRL,
        KEY_RIGHTALT,
        KEY_LEFTMETA,
        KEY_RIGHTMETA,

        #[strum(disabled)]
        KEY_OTHER(u64),
    }

    impl From<u64> for KeybdKey {
        // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
        fn from(code: u64) -> KeybdKey {
            match code {
                0x08 => KeybdKey::KEY_BACKSPACE,
                0x09 => KeybdKey::KEY_TAB,
                0x0D => KeybdKey::KEY_ENTER,
                0x1B => KeybdKey::KEY_ESC,
                0x20 => KeybdKey::KEY_SPACE,
                0x21 => KeybdKey::KEY_PAGEUP,
                0x22 => KeybdKey::KEY_PAGEDOWN,
                0x23 => KeybdKey::KEY_END,
                0x24 => KeybdKey::KEY_HOME,
                0x25 => KeybdKey::KEY_LEFT,
                0x26 => KeybdKey::KEY_UP,
                0x27 => KeybdKey::KEY_RIGHT,
                0x28 => KeybdKey::KEY_DOWN,
                0x2D => KeybdKey::KEY_INSERT,
                0x2E => KeybdKey::KEY_DELETE,
                0x30 => KeybdKey::KEY_0,
                0x31 => KeybdKey::KEY_1,
                0x32 => KeybdKey::KEY_2,
                0x33 => KeybdKey::KEY_3,
                0x34 => KeybdKey::KEY_4,
                0x35 => KeybdKey::KEY_5,
                0x36 => KeybdKey::KEY_6,
                0x37 => KeybdKey::KEY_7,
                0x38 => KeybdKey::KEY_8,
                0x39 => KeybdKey::KEY_9,
                0x41 => KeybdKey::KEY_A,
                0x42 => KeybdKey::KEY_B,
                0x43 => KeybdKey::KEY_C,
                0x44 => KeybdKey::KEY_D,
                0x45 => KeybdKey::KEY_E,
                0x46 => KeybdKey::KEY_F,
                0x47 => KeybdKey::KEY_G,
                0x48 => KeybdKey::KEY_H,
                0x49 => KeybdKey::KEY_I,
                0x4A => KeybdKey::KEY_J,
                0x4B => KeybdKey::KEY_K,
                0x4C => KeybdKey::KEY_L,
                0x4D => KeybdKey::KEY_M,
                0x4E => KeybdKey::KEY_N,
                0x4F => KeybdKey::KEY_O,
                0x50 => KeybdKey::KEY_P,
                0x51 => KeybdKey::KEY_Q,
                0x52 => KeybdKey::KEY_R,
                0x53 => KeybdKey::KEY_S,
                0x54 => KeybdKey::KEY_T,
                0x55 => KeybdKey::KEY_U,
                0x56 => KeybdKey::KEY_V,
                0x57 => KeybdKey::KEY_W,
                0x58 => KeybdKey::KEY_X,
                0x59 => KeybdKey::KEY_Y,
                0x5A => KeybdKey::KEY_Z,
                0x5B => KeybdKey::KEY_LEFTMETA,
                0x5C => KeybdKey::KEY_RIGHTMETA,
                0x60 => KeybdKey::KEY_KP0,
                0x61 => KeybdKey::KEY_KP1,
                0x62 => KeybdKey::KEY_KP2,
                0x63 => KeybdKey::KEY_KP3,
                0x64 => KeybdKey::KEY_KP4,
                0x65 => KeybdKey::KEY_KP5,
                0x66 => KeybdKey::KEY_KP6,
                0x67 => KeybdKey::KEY_KP7,
                0x68 => KeybdKey::KEY_KP8,
                0x69 => KeybdKey::KEY_KP9,
                0x70 => KeybdKey::KEY_F1,
                0x71 => KeybdKey::KEY_F2,
                0x72 => KeybdKey::KEY_F3,
                0x73 => KeybdKey::KEY_F4,
                0x74 => KeybdKey::KEY_F5,
                0x75 => KeybdKey::KEY_F6,
                0x76 => KeybdKey::KEY_F7,
                0x77 => KeybdKey::KEY_F8,
                0x78 => KeybdKey::KEY_F9,
                0x79 => KeybdKey::KEY_F10,
                0x7A => KeybdKey::KEY_F11,
                0x7B => KeybdKey::KEY_F12,
                0x90 => KeybdKey::KEY_NUMLOCK,
                0x91 => KeybdKey::KEY_SCROLLLOCK,
                0x14 => KeybdKey::KEY_CAPSLOCK,
                0xA0 => KeybdKey::KEY_LEFTSHIFT,
                0xA1 => KeybdKey::KEY_RIGHTSHIFT,
                0xA2 => KeybdKey::KEY_LEFTCTRL,
                0xA3 => KeybdKey::KEY_RIGHTCTRL,
                0xA4 => KeybdKey::KEY_LEFTALT,
                0xA5 => KeybdKey::KEY_RIGHTALT,
                0xAD => KeybdKey::KEY_MUTE,
                0xAE => KeybdKey::KEY_VOLUMEDOWN,
                0xAF => KeybdKey::KEY_VOLUMEUP,
                0xB0 => KeybdKey::KEY_NEXTSONG,
                0xB1 => KeybdKey::KEY_PREVIOUSSONG,
                0xB2 => KeybdKey::KEY_STOPCD,
                0xB3 => KeybdKey::KEY_PLAYPAUSE,
                0xC0 => KeybdKey::KEY_GRAVE,
                0xBF => KeybdKey::KEY_SLASH,
                0xDC => KeybdKey::KEY_BACKSLASH,
                0xBC => KeybdKey::KEY_COMMA,
                0xBE => KeybdKey::KEY_DOT,
                0xBD => KeybdKey::KEY_MINUS,
                0xDE => KeybdKey::KEY_APOSTROPHE,
                0xBA => KeybdKey::KEY_SEMICOLON,
                0xDB => KeybdKey::KEY_LEFTBRACE,
                0xDD => KeybdKey::KEY_RIGHTBRACE,
                0xBB => KeybdKey::KEY_EQUAL,
                _ => KeybdKey::KEY_OTHER(code),
            }
        }
    }

    impl std::fmt::Display for KeybdKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    KeybdKey::KEY_BACKSPACE => "Backspace",
                    KeybdKey::KEY_TAB => "Tab",
                    KeybdKey::KEY_ENTER => "Enter",
                    KeybdKey::KEY_ESC => "Escape",
                    KeybdKey::KEY_SPACE => "Space",
                    KeybdKey::KEY_PAGEUP => "PageUp",
                    KeybdKey::KEY_PAGEDOWN => "PageDown",
                    KeybdKey::KEY_END => "End",
                    KeybdKey::KEY_HOME => "Home",
                    KeybdKey::KEY_LEFT => "Left",
                    KeybdKey::KEY_UP => "Up",
                    KeybdKey::KEY_RIGHT => "Right",
                    KeybdKey::KEY_DOWN => "Down",
                    KeybdKey::KEY_INSERT => "Insert",
                    KeybdKey::KEY_DELETE => "Delete",
                    KeybdKey::KEY_0 => "0",
                    KeybdKey::KEY_1 => "1",
                    KeybdKey::KEY_2 => "2",
                    KeybdKey::KEY_3 => "3",
                    KeybdKey::KEY_4 => "4",
                    KeybdKey::KEY_5 => "5",
                    KeybdKey::KEY_6 => "6",
                    KeybdKey::KEY_7 => "7",
                    KeybdKey::KEY_8 => "8",
                    KeybdKey::KEY_9 => "9",
                    KeybdKey::KEY_A => "a",
                    KeybdKey::KEY_B => "b",
                    KeybdKey::KEY_C => "c",
                    KeybdKey::KEY_D => "d",
                    KeybdKey::KEY_E => "e",
                    KeybdKey::KEY_F => "f",
                    KeybdKey::KEY_G => "g",
                    KeybdKey::KEY_H => "h",
                    KeybdKey::KEY_I => "i",
                    KeybdKey::KEY_J => "j",
                    KeybdKey::KEY_K => "k",
                    KeybdKey::KEY_L => "l",
                    KeybdKey::KEY_M => "m",
                    KeybdKey::KEY_N => "n",
                    KeybdKey::KEY_O => "o",
                    KeybdKey::KEY_P => "p",
                    KeybdKey::KEY_Q => "q",
                    KeybdKey::KEY_R => "r",
                    KeybdKey::KEY_S => "s",
                    KeybdKey::KEY_T => "t",
                    KeybdKey::KEY_U => "u",
                    KeybdKey::KEY_V => "v",
                    KeybdKey::KEY_W => "w",
                    KeybdKey::KEY_X => "x",
                    KeybdKey::KEY_Y => "y",
                    KeybdKey::KEY_Z => "z",
                    KeybdKey::KEY_LEFTMETA => "LeftWindows",
                    KeybdKey::KEY_RIGHTMETA => "RightWindows",
                    KeybdKey::KEY_KP0 => "NumPad0",
                    KeybdKey::KEY_KP1 => "NumPad1",
                    KeybdKey::KEY_KP2 => "NumPad2",
                    KeybdKey::KEY_KP3 => "NumPad3",
                    KeybdKey::KEY_KP4 => "NumPad4",
                    KeybdKey::KEY_KP5 => "NumPad5",
                    KeybdKey::KEY_KP6 => "NumPad6",
                    KeybdKey::KEY_KP7 => "NumPad7",
                    KeybdKey::KEY_KP8 => "NumPad8",
                    KeybdKey::KEY_KP9 => "NumPad9",
                    KeybdKey::KEY_F1 => "F1",
                    KeybdKey::KEY_F2 => "F2",
                    KeybdKey::KEY_F3 => "F3",
                    KeybdKey::KEY_F4 => "F4",
                    KeybdKey::KEY_F5 => "F5",
                    KeybdKey::KEY_F6 => "F6",
                    KeybdKey::KEY_F7 => "F7",
                    KeybdKey::KEY_F8 => "F8",
                    KeybdKey::KEY_F9 => "F9",
                    KeybdKey::KEY_F10 => "F10",
                    KeybdKey::KEY_F11 => "F11",
                    KeybdKey::KEY_F12 => "F12",
                    KeybdKey::KEY_NUMLOCK => "NumLock",
                    KeybdKey::KEY_SCROLLLOCK => "ScrollLock",
                    KeybdKey::KEY_CAPSLOCK => "CapsLock",
                    KeybdKey::KEY_LEFTSHIFT => "LeftShift",
                    KeybdKey::KEY_RIGHTSHIFT => "RightShift",
                    KeybdKey::KEY_LEFTCTRL => "LeftControl",
                    KeybdKey::KEY_RIGHTCTRL => "RightControl",
                    KeybdKey::KEY_LEFTALT => "LeftAlt",
                    KeybdKey::KEY_RIGHTALT => "RightAlt",
                    KeybdKey::KEY_MUTE => "VolumeMute",
                    KeybdKey::KEY_VOLUMEDOWN => "VolumeDown",
                    KeybdKey::KEY_VOLUMEUP => "VolumeUp",
                    KeybdKey::KEY_NEXTSONG => "MediaNext",
                    KeybdKey::KEY_PREVIOUSSONG => "MediaPrevious",
                    KeybdKey::KEY_STOPCD => "MediaStop",
                    KeybdKey::KEY_PLAYPAUSE => "MediaPlay",
                    KeybdKey::KEY_GRAVE => "Backquote",
                    KeybdKey::KEY_SLASH => "Slash",
                    KeybdKey::KEY_BACKSLASH => "Backslash",
                    KeybdKey::KEY_COMMA => "Comma",
                    KeybdKey::KEY_DOT => "Period",
                    KeybdKey::KEY_MINUS => "Minus",
                    KeybdKey::KEY_APOSTROPHE => "QuoteKey",
                    KeybdKey::KEY_SEMICOLON => "Semicolon",
                    KeybdKey::KEY_LEFTBRACE => "LeftBracket",
                    KeybdKey::KEY_RIGHTBRACE => "RightBracket",
                    KeybdKey::KEY_EQUAL => "Equal",
                    KeybdKey::KEY_KPMINUS => "KeypadMinus",
                    KeybdKey::KEY_KPPLUS => "KeypadPlus",
                    KeybdKey::KEY_KPDOT => "KeypadDot",
                    KeybdKey::KEY_OTHER(code) => return write!(f, "OtherKey({code})"),
                }
            )
        }
    }

    #[derive(Debug, Error)]
    pub enum ParseError {
        #[error("Unable to parse the keycode value")]
        ParseIntError {
            #[from]
            source: std::num::ParseIntError,
            backtrace: std::backtrace::Backtrace,
        },
        #[error("Unknown format '{val}'")]
        UnknownFormat {
            val: String,
            backtrace: std::backtrace::Backtrace,
        },
    }

    impl std::str::FromStr for KeybdKey {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s_lower = s.to_lowercase();
            if let Some(k) = keyboard_canonical_names_lower().get(&s_lower) {
                return Ok(*k);
            }
            match s_lower.as_str() {
                "leftwindows" => return Ok(KeybdKey::LSuper),
                "leftcommand" => return Ok(KeybdKey::LSuper),
                "rightwindows" => return Ok(KeybdKey::RSuper),
                "rightcommand" => return Ok(KeybdKey::RSuper),
                _ => {}
            }
            if let Some(caps) = other_key_regex().captures(s) {
                let v = &caps[1]
                    .parse::<u64>()
                    .map_err(|err| Into::<ParseError>::into(err))?;
                return Ok(KeybdKey::OtherKey(*v));
            }

            Err(ParseError::UnknownFormat {
                val: s.to_string(),
                backtrace: std::backtrace::Backtrace::capture(),
            })
        }
    }

    pub fn get_all_keys_code() -> Vec<KeyCode> {
        ALL_KEYS.into_iter().collect()
    }

    #[derive(Debug)]
    pub struct KeyEventHolder {
        minimum_delay: Duration,
        container: HashMap<u16, SystemTime>,
    }

    impl KeyEventHolder {
        pub fn new(minimum_delay: u64) -> KeyEventHolder {
            KeyEventHolder {
                minimum_delay: Duration::from_millis(minimum_delay),
                container: HashMap::new(),
            }
        }

        fn insert_event(&mut self, key_code: u16, timestamp: SystemTime) {
            self.container.insert(key_code, timestamp);
        }

        fn remove_event(&mut self, key_code: u16) {
            self.container.remove(&key_code);
        }

        fn last_timestamp(&self, key_code: u16) -> Option<&SystemTime> {
            self.container.get(&key_code)
        }
    }

    pub fn should_skip(
        ev: &KeyEvent,
        key_holder: &mut KeyEventHolder,
        config_holder: &ConfigHolder,
    ) -> bool {
        if ev.value() == 2 {
            return false;
        }
        if !config_holder.keys.contains(ev.code()) {
            return false;
        }
        match key_holder.last_timestamp(ev.code().code()) {
            Some(&timestamp) => {
                let should_skip =
                    ev.timestamp().duration_since(timestamp).unwrap() <= key_holder.minimum_delay;
                if ev.value() == 1 {
                    if !should_skip {
                        key_holder.remove_event(ev.code().code());
                        return false;
                    }
                } else {
                    return true;
                }
                should_skip
            }
            None => {
                if ev.value() == 0 {
                    key_holder.insert_event(ev.code().code(), ev.timestamp());
                }
                false
            }
        }
    }

    pub fn list_devices() -> Vec<Device> {
        evdev::enumerate().map(|d| d.1).collect::<Vec<Device>>()
    }

    pub fn receive_event(device: &mut Device) -> FetchEventsSynced {
        device.fetch_events().unwrap()
    }

    pub fn emit_key_event(code: u16, value: i32, virtual_device: &mut VirtualDevice) {
        let key_event = *KeyEvent::new(KeyCode(code), value);
        virtual_device.emit(&[key_event]).unwrap();
    }

    pub fn create_virtual_device() -> VirtualDevice {
        let mut keys = AttributeSet::<KeyCode>::new();
        for key in get_all_keys_code() {
            keys.insert(key);
        }
        VirtualDevice::builder()
            .unwrap()
            .name("Virtual Keyboard")
            .with_keys(&keys)
            .unwrap()
            .build()
            .unwrap()
    }

    pub fn combine_u16_to_u32(high: u16, low: u16) -> u32 {
        ((high as u32) << 16) | (low as u32)
    }

    pub fn split_u32_to_u16(value: u32) -> (u16, u16) {
        let high = (value >> 16) as u16;
        let low = (value & 0xFFFF) as u16;
        (high, low)
    }
}

pub mod command_line {
    use std::path::PathBuf;

    use clap::{Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[command(
        version,
        about,
        long_about = "A utility designed to eliminate duplicate keystrokes by setting a delay between key presses when typing."
    )]
    pub struct Cli {
        /// Path to the config file (if not provided, it will create a config file in the current directory or use an existing one)
        #[arg(short, long, value_name = "path")]
        pub config_path: Option<PathBuf>,

        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand, Debug)]
    pub enum Commands {
        /// List all connected devices
        #[clap(name = "list")]
        ListDevices,
        /// select the device to listen events
        Select {
            /// device number from list option [0-n]
            device: String,
        },
    }
}

pub mod config {
    use crate::device::windows::debounce::KeybdKey;
    use crate::device::windows::debounce::get_all_keys_code;
    use config::{Config, File, FileFormat};
    use evdev::{AttributeSet, KeyCode};
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::io::Write;
    use std::path::PathBuf;

    const AVAILABLE_KEYS: [(&str, KeybdKey); 95] = [
        ("KEY_ESC", KeybdKey::KEY_ESC),
        ("KEY_1", KeybdKey::KEY_1),
        ("KEY_2", KeybdKey::KEY_2),
        ("KEY_3", KeybdKey::KEY_3),
        ("KEY_4", KeybdKey::KEY_4),
        ("KEY_5", KeybdKey::KEY_5),
        ("KEY_6", KeybdKey::KEY_6),
        ("KEY_7", KeybdKey::KEY_7),
        ("KEY_8", KeybdKey::KEY_8),
        ("KEY_9", KeybdKey::KEY_9),
        ("KEY_0", KeybdKey::KEY_0),
        ("KEY_MINUS", KeybdKey::KEY_MINUS),
        ("KEY_EQUAL", KeybdKey::KEY_EQUAL),
        ("KEY_BACKSPACE", KeybdKey::KEY_BACKSPACE),
        ("KEY_TAB", KeybdKey::KEY_TAB),
        ("KEY_Q", KeybdKey::KEY_Q),
        ("KEY_W", KeybdKey::KEY_W),
        ("KEY_E", KeybdKey::KEY_E),
        ("KEY_R", KeybdKey::KEY_R),
        ("KEY_T", KeybdKey::KEY_T),
        ("KEY_Y", KeybdKey::KEY_Y),
        ("KEY_U", KeybdKey::KEY_U),
        ("KEY_I", KeybdKey::KEY_I),
        ("KEY_O", KeybdKey::KEY_O),
        ("KEY_P", KeybdKey::KEY_P),
        ("KEY_LEFTBRACE", KeybdKey::KEY_LEFTBRACE),
        ("KEY_RIGHTBRACE", KeybdKey::KEY_RIGHTBRACE),
        ("KEY_ENTER", KeybdKey::KEY_ENTER),
        ("KEY_LEFTCTRL", KeybdKey::KEY_LEFTCTRL),
        ("KEY_A", KeybdKey::KEY_A),
        ("KEY_S", KeybdKey::KEY_S),
        ("KEY_D", KeybdKey::KEY_D),
        ("KEY_F", KeybdKey::KEY_F),
        ("KEY_G", KeybdKey::KEY_G),
        ("KEY_H", KeybdKey::KEY_H),
        ("KEY_J", KeybdKey::KEY_J),
        ("KEY_K", KeybdKey::KEY_K),
        ("KEY_L", KeybdKey::KEY_L),
        ("KEY_SEMICOLON", KeybdKey::KEY_SEMICOLON),
        ("KEY_APOSTROPHE", KeybdKey::KEY_APOSTROPHE),
        ("KEY_GRAVE", KeybdKey::KEY_GRAVE),
        ("KEY_LEFTSHIFT", KeybdKey::KEY_LEFTSHIFT),
        ("KEY_BACKSLASH", KeybdKey::KEY_BACKSLASH),
        ("KEY_Z", KeybdKey::KEY_Z),
        ("KEY_X", KeybdKey::KEY_X),
        ("KEY_C", KeybdKey::KEY_C),
        ("KEY_V", KeybdKey::KEY_V),
        ("KEY_B", KeybdKey::KEY_B),
        ("KEY_N", KeybdKey::KEY_N),
        ("KEY_M", KeybdKey::KEY_M),
        ("KEY_COMMA", KeybdKey::KEY_COMMA),
        ("KEY_DOT", KeybdKey::KEY_DOT),
        ("KEY_SLASH", KeybdKey::KEY_SLASH),
        ("KEY_RIGHTSHIFT", KeybdKey::KEY_RIGHTSHIFT),
        ("KEY_KPASTERISK", KeybdKey::KEY_KPASTERISK),
        ("KEY_LEFTALT", KeybdKey::KEY_LEFTALT),
        ("KEY_SPACE", KeybdKey::KEY_SPACE),
        ("KEY_CAPSLOCK", KeybdKey::KEY_CAPSLOCK),
        ("KEY_F1", KeybdKey::KEY_F1),
        ("KEY_F2", KeybdKey::KEY_F2),
        ("KEY_F3", KeybdKey::KEY_F3),
        ("KEY_F4", KeybdKey::KEY_F4),
        ("KEY_F5", KeybdKey::KEY_F5),
        ("KEY_F6", KeybdKey::KEY_F6),
        ("KEY_F7", KeybdKey::KEY_F7),
        ("KEY_F8", KeybdKey::KEY_F8),
        ("KEY_F9", KeybdKey::KEY_F9),
        ("KEY_F10", KeybdKey::KEY_F10),
        ("KEY_NUMLOCK", KeybdKey::KEY_NUMLOCK),
        ("KEY_SCROLLLOCK", KeybdKey::KEY_SCROLLLOCK),
        ("KEY_F11", KeybdKey::KEY_F11),
        ("KEY_F12", KeybdKey::KEY_F12),
        ("KEY_SYSRQ", KeybdKey::KEY_SYSRQ),
        ("KEY_RIGHTALT", KeybdKey::KEY_RIGHTALT),
        ("KEY_HOME", KeybdKey::KEY_HOME),
        ("KEY_UP", KeybdKey::KEY_UP),
        ("KEY_PAGEUP", KeybdKey::KEY_PAGEUP),
        ("KEY_LEFT", KeybdKey::KEY_LEFT),
        ("KEY_RIGHT", KeybdKey::KEY_RIGHT),
        ("KEY_END", KeybdKey::KEY_END),
        ("KEY_DOWN", KeybdKey::KEY_DOWN),
        ("KEY_PAGEDOWN", KeybdKey::KEY_PAGEDOWN),
        ("KEY_INSERT", KeybdKey::KEY_INSERT),
        ("KEY_DELETE", KeybdKey::KEY_DELETE),
        ("KEY_PAUSE", KeybdKey::KEY_PAUSE),
        ("KEY_NUMERIC_0", KeybdKey::KEY_NUMERIC_0),
        ("KEY_NUMERIC_1", KeybdKey::KEY_NUMERIC_1),
        ("KEY_NUMERIC_2", KeybdKey::KEY_NUMERIC_2),
        ("KEY_NUMERIC_3", KeybdKey::KEY_NUMERIC_3),
        ("KEY_NUMERIC_4", KeybdKey::KEY_NUMERIC_4),
        ("KEY_NUMERIC_5", KeybdKey::KEY_NUMERIC_5),
        ("KEY_NUMERIC_6", KeybdKey::KEY_NUMERIC_6),
        ("KEY_NUMERIC_7", KeybdKey::KEY_NUMERIC_7),
        ("KEY_NUMERIC_8", KeybdKey::KEY_NUMERIC_8),
        ("KEY_NUMERIC_9", KeybdKey::KEY_NUMERIC_9),
    ];

    pub struct ConfigHolder {
        pub keys: Vec<KeybdKey>,
        pub device_id: u32,
        pub device_name: String,
        pub delay_ms: u64,
    }

    impl Display for ConfigHolder {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let keys = AVAILABLE_KEYS
                .into_iter()
                .filter(|(_, v)| self.keys.contains(*v))
                .map(|(k, _)| k)
                .collect::<Vec<_>>()
                .join(",");
            write!(
                f,
                "keys={keys}\ndelay_ms={}\ndevice_id={}\ndevice_name={}",
                self.delay_ms, self.device_id, self.device_name
            )
        }
    }

    pub fn load_config(file: &PathBuf) -> ConfigHolder {
        if !file.exists() {
            std::fs::File::create(file).expect("Failed to create config file");
        }
        let file = File::new(file.to_str().unwrap(), FileFormat::Ini);

        let settings = Config::builder().add_source(file).build().unwrap();

        let settings = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

        let keys = match settings.get("keys") {
            Some(keys) => get_keys_code(keys),
            None => get_all_keys_code(),
        };

        let keys: AttributeSet<KeyCode> = keys.iter().collect();

        let delay_ms = match settings.get("delay_ms") {
            Some(value) => value.parse::<u64>().unwrap(),
            None => 85,
        };

        let device_id = match settings.get("device_id") {
            Some(id) => id.parse::<u32>().expect("Invalid device id"),
            None => 0,
        };

        let device_name = match settings.get("device_name") {
            Some(name) => name.to_owned(),
            None => "unknown".to_owned(),
        };

        ConfigHolder {
            keys,
            device_id,
            device_name,
            delay_ms,
        }
    }

    pub fn save_config_to_path(path: &PathBuf, config: &ConfigHolder) {
        let mut config_file = std::fs::File::create(path).expect("Failed to create config file");
        config_file
            .write_all(config.to_string().as_bytes())
            .expect("Failed to write to config file");
    }

    fn get_keys_code(keys: &str) -> Vec<KeyCode> {
        let available_keys_map = AVAILABLE_KEYS
            .into_iter()
            .collect::<HashMap<&str, KeyCode>>();
        let keys_code = keys
            .split(',')
            .map(|k| (k, available_keys_map.get(k)))
            .collect::<Vec<_>>();

        if keys_code.iter().all(|(_, v)| v.is_some()) {
            keys_code.into_iter().map(|(_, v)| *v.unwrap()).collect()
        } else {
            let invalid_keys = keys_code
                .into_iter()
                .filter(|&(_, v)| v.is_none())
                .map(|(k, _)| k)
                .collect::<Vec<_>>();
            panic!("invalid key codes: {:?}", invalid_keys.join(","));
        }
    }
}
