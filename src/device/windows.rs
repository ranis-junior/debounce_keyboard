pub mod debounce {
    use crate::device::windows::config::ConfigHolder;
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};
    use strum::{EnumIter, IntoEnumIterator};

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
    #[allow(non_camel_case_types)]
    pub enum MappedKey {
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

    impl From<u64> for MappedKey {
        // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
        fn from(code: u64) -> MappedKey {
            match code {
                0x08 => MappedKey::KEY_BACKSPACE,
                0x09 => MappedKey::KEY_TAB,
                0x0D => MappedKey::KEY_ENTER,
                0x1B => MappedKey::KEY_ESC,
                0x20 => MappedKey::KEY_SPACE,
                0x21 => MappedKey::KEY_PAGEUP,
                0x22 => MappedKey::KEY_PAGEDOWN,
                0x23 => MappedKey::KEY_END,
                0x24 => MappedKey::KEY_HOME,
                0x25 => MappedKey::KEY_LEFT,
                0x26 => MappedKey::KEY_UP,
                0x27 => MappedKey::KEY_RIGHT,
                0x28 => MappedKey::KEY_DOWN,
                0x2D => MappedKey::KEY_INSERT,
                0x2E => MappedKey::KEY_DELETE,
                0x30 => MappedKey::KEY_0,
                0x31 => MappedKey::KEY_1,
                0x32 => MappedKey::KEY_2,
                0x33 => MappedKey::KEY_3,
                0x34 => MappedKey::KEY_4,
                0x35 => MappedKey::KEY_5,
                0x36 => MappedKey::KEY_6,
                0x37 => MappedKey::KEY_7,
                0x38 => MappedKey::KEY_8,
                0x39 => MappedKey::KEY_9,
                0x41 => MappedKey::KEY_A,
                0x42 => MappedKey::KEY_B,
                0x43 => MappedKey::KEY_C,
                0x44 => MappedKey::KEY_D,
                0x45 => MappedKey::KEY_E,
                0x46 => MappedKey::KEY_F,
                0x47 => MappedKey::KEY_G,
                0x48 => MappedKey::KEY_H,
                0x49 => MappedKey::KEY_I,
                0x4A => MappedKey::KEY_J,
                0x4B => MappedKey::KEY_K,
                0x4C => MappedKey::KEY_L,
                0x4D => MappedKey::KEY_M,
                0x4E => MappedKey::KEY_N,
                0x4F => MappedKey::KEY_O,
                0x50 => MappedKey::KEY_P,
                0x51 => MappedKey::KEY_Q,
                0x52 => MappedKey::KEY_R,
                0x53 => MappedKey::KEY_S,
                0x54 => MappedKey::KEY_T,
                0x55 => MappedKey::KEY_U,
                0x56 => MappedKey::KEY_V,
                0x57 => MappedKey::KEY_W,
                0x58 => MappedKey::KEY_X,
                0x59 => MappedKey::KEY_Y,
                0x5A => MappedKey::KEY_Z,
                0x5B => MappedKey::KEY_LEFTMETA,
                0x5C => MappedKey::KEY_RIGHTMETA,
                0x60 => MappedKey::KEY_KP0,
                0x61 => MappedKey::KEY_KP1,
                0x62 => MappedKey::KEY_KP2,
                0x63 => MappedKey::KEY_KP3,
                0x64 => MappedKey::KEY_KP4,
                0x65 => MappedKey::KEY_KP5,
                0x66 => MappedKey::KEY_KP6,
                0x67 => MappedKey::KEY_KP7,
                0x68 => MappedKey::KEY_KP8,
                0x69 => MappedKey::KEY_KP9,
                0x70 => MappedKey::KEY_F1,
                0x71 => MappedKey::KEY_F2,
                0x72 => MappedKey::KEY_F3,
                0x73 => MappedKey::KEY_F4,
                0x74 => MappedKey::KEY_F5,
                0x75 => MappedKey::KEY_F6,
                0x76 => MappedKey::KEY_F7,
                0x77 => MappedKey::KEY_F8,
                0x78 => MappedKey::KEY_F9,
                0x79 => MappedKey::KEY_F10,
                0x7A => MappedKey::KEY_F11,
                0x7B => MappedKey::KEY_F12,
                0x90 => MappedKey::KEY_NUMLOCK,
                0x91 => MappedKey::KEY_SCROLLLOCK,
                0x14 => MappedKey::KEY_CAPSLOCK,
                0xA0 => MappedKey::KEY_LEFTSHIFT,
                0xA1 => MappedKey::KEY_RIGHTSHIFT,
                0xA2 => MappedKey::KEY_LEFTCTRL,
                0xA3 => MappedKey::KEY_RIGHTCTRL,
                0xA4 => MappedKey::KEY_LEFTALT,
                0xA5 => MappedKey::KEY_RIGHTALT,
                0xAD => MappedKey::KEY_MUTE,
                0xAE => MappedKey::KEY_VOLUMEDOWN,
                0xAF => MappedKey::KEY_VOLUMEUP,
                0xB0 => MappedKey::KEY_NEXTSONG,
                0xB1 => MappedKey::KEY_PREVIOUSSONG,
                0xB2 => MappedKey::KEY_STOPCD,
                0xB3 => MappedKey::KEY_PLAYPAUSE,
                0xC0 => MappedKey::KEY_GRAVE,
                0xBF => MappedKey::KEY_SLASH,
                0xDC => MappedKey::KEY_BACKSLASH,
                0xBC => MappedKey::KEY_COMMA,
                0xBE => MappedKey::KEY_DOT,
                0xBD => MappedKey::KEY_MINUS,
                0xDE => MappedKey::KEY_APOSTROPHE,
                0xBA => MappedKey::KEY_SEMICOLON,
                0xDB => MappedKey::KEY_LEFTBRACE,
                0xDD => MappedKey::KEY_RIGHTBRACE,
                0xBB => MappedKey::KEY_EQUAL,
                _ => MappedKey::KEY_OTHER(code),
            }
        }
    }

    pub fn get_all_keys_code() -> Vec<MappedKey> {
        MappedKey::iter().collect()
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

    #[derive(Debug)]
    pub struct KeyEvent {
        pub keycode: u16,
        pub value: i32,
        pub timestamp: SystemTime,
    }

    impl KeyEvent {
        pub fn new(keycode: u16, value: i32, timestamp: SystemTime) -> KeyEvent {
            KeyEvent {
                keycode,
                value,
                timestamp,
            }
        }
    }

    pub struct Device {
        pub vendor: u16,
        pub product: u16,
        pub device_internal: DeviceWin,
    }

    impl Device {
        pub fn new(vendor: u16, product: u16, device_internal: DeviceWin) -> Device {
            Device {
                vendor,
                product,
                device_internal,
            }
        }

        pub fn grab(&mut self) {
            self.device_internal.grab().expect("Error on grab device");
        }

        #[allow(dead_code)]
        pub fn ungrab(&mut self) {
            self.device_internal
                .ungrab()
                .expect("Error on ungrab device");
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
    
    use windows::Win32::UI::Input::{ }

    pub fn list_devices() -> Vec<Device> {
        ::enumerate()
            .map(|(_, device)| {
                Device::new(
                    device.input_id().vendor(),
                    device.input_id().product(),
                    device,
                )
            })
            .collect::<Vec<Device>>()
    }

    pub fn receive_event(device: &mut Device) -> Vec<KeyEvent> {
        let result: Vec<InputEvent> = device.device_internal.fetch_events().unwrap().collect();
        result
            .into_iter()
            .filter_map(|event| {
                if let EventSummary::Key(event, _, _) = event.destructure() {
                    return Some(KeyEvent::new(
                        event.code().code(),
                        event.value(),
                        event.timestamp(),
                    ));
                }
                None
            })
            .collect()
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
    use crate::device::windows::debounce::get_all_keys_code;
    use crate::device::windows::debounce::MappedKey;
    use config::{Config, File, FileFormat};
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::io::Write;
    use std::path::PathBuf;

    const AVAILABLE_KEYS: [(&str, MappedKey); 95] = [
        ("KEY_ESC", MappedKey::KEY_ESC),
        ("KEY_1", MappedKey::KEY_1),
        ("KEY_2", MappedKey::KEY_2),
        ("KEY_3", MappedKey::KEY_3),
        ("KEY_4", MappedKey::KEY_4),
        ("KEY_5", MappedKey::KEY_5),
        ("KEY_6", MappedKey::KEY_6),
        ("KEY_7", MappedKey::KEY_7),
        ("KEY_8", MappedKey::KEY_8),
        ("KEY_9", MappedKey::KEY_9),
        ("KEY_0", MappedKey::KEY_0),
        ("KEY_MINUS", MappedKey::KEY_MINUS),
        ("KEY_EQUAL", MappedKey::KEY_EQUAL),
        ("KEY_BACKSPACE", MappedKey::KEY_BACKSPACE),
        ("KEY_TAB", MappedKey::KEY_TAB),
        ("KEY_Q", MappedKey::KEY_Q),
        ("KEY_W", MappedKey::KEY_W),
        ("KEY_E", MappedKey::KEY_E),
        ("KEY_R", MappedKey::KEY_R),
        ("KEY_T", MappedKey::KEY_T),
        ("KEY_Y", MappedKey::KEY_Y),
        ("KEY_U", MappedKey::KEY_U),
        ("KEY_I", MappedKey::KEY_I),
        ("KEY_O", MappedKey::KEY_O),
        ("KEY_P", MappedKey::KEY_P),
        ("KEY_LEFTBRACE", MappedKey::KEY_LEFTBRACE),
        ("KEY_RIGHTBRACE", MappedKey::KEY_RIGHTBRACE),
        ("KEY_ENTER", MappedKey::KEY_ENTER),
        ("KEY_LEFTCTRL", MappedKey::KEY_LEFTCTRL),
        ("KEY_A", MappedKey::KEY_A),
        ("KEY_S", MappedKey::KEY_S),
        ("KEY_D", MappedKey::KEY_D),
        ("KEY_F", MappedKey::KEY_F),
        ("KEY_G", MappedKey::KEY_G),
        ("KEY_H", MappedKey::KEY_H),
        ("KEY_J", MappedKey::KEY_J),
        ("KEY_K", MappedKey::KEY_K),
        ("KEY_L", MappedKey::KEY_L),
        ("KEY_SEMICOLON", MappedKey::KEY_SEMICOLON),
        ("KEY_APOSTROPHE", MappedKey::KEY_APOSTROPHE),
        ("KEY_GRAVE", MappedKey::KEY_GRAVE),
        ("KEY_LEFTSHIFT", MappedKey::KEY_LEFTSHIFT),
        ("KEY_BACKSLASH", MappedKey::KEY_BACKSLASH),
        ("KEY_Z", MappedKey::KEY_Z),
        ("KEY_X", MappedKey::KEY_X),
        ("KEY_C", MappedKey::KEY_C),
        ("KEY_V", MappedKey::KEY_V),
        ("KEY_B", MappedKey::KEY_B),
        ("KEY_N", MappedKey::KEY_N),
        ("KEY_M", MappedKey::KEY_M),
        ("KEY_COMMA", MappedKey::KEY_COMMA),
        ("KEY_DOT", MappedKey::KEY_DOT),
        ("KEY_SLASH", MappedKey::KEY_SLASH),
        ("KEY_RIGHTSHIFT", MappedKey::KEY_RIGHTSHIFT),
        ("KEY_KPASTERISK", MappedKey::KEY_KPASTERISK),
        ("KEY_LEFTALT", MappedKey::KEY_LEFTALT),
        ("KEY_SPACE", MappedKey::KEY_SPACE),
        ("KEY_CAPSLOCK", MappedKey::KEY_CAPSLOCK),
        ("KEY_F1", MappedKey::KEY_F1),
        ("KEY_F2", MappedKey::KEY_F2),
        ("KEY_F3", MappedKey::KEY_F3),
        ("KEY_F4", MappedKey::KEY_F4),
        ("KEY_F5", MappedKey::KEY_F5),
        ("KEY_F6", MappedKey::KEY_F6),
        ("KEY_F7", MappedKey::KEY_F7),
        ("KEY_F8", MappedKey::KEY_F8),
        ("KEY_F9", MappedKey::KEY_F9),
        ("KEY_F10", MappedKey::KEY_F10),
        ("KEY_NUMLOCK", MappedKey::KEY_NUMLOCK),
        ("KEY_SCROLLLOCK", MappedKey::KEY_SCROLLLOCK),
        ("KEY_F11", MappedKey::KEY_F11),
        ("KEY_F12", MappedKey::KEY_F12),
        ("KEY_SYSRQ", MappedKey::KEY_SYSRQ),
        ("KEY_RIGHTALT", MappedKey::KEY_RIGHTALT),
        ("KEY_HOME", MappedKey::KEY_HOME),
        ("KEY_UP", MappedKey::KEY_UP),
        ("KEY_PAGEUP", MappedKey::KEY_PAGEUP),
        ("KEY_LEFT", MappedKey::KEY_LEFT),
        ("KEY_RIGHT", MappedKey::KEY_RIGHT),
        ("KEY_END", MappedKey::KEY_END),
        ("KEY_DOWN", MappedKey::KEY_DOWN),
        ("KEY_PAGEDOWN", MappedKey::KEY_PAGEDOWN),
        ("KEY_INSERT", MappedKey::KEY_INSERT),
        ("KEY_DELETE", MappedKey::KEY_DELETE),
        ("KEY_PAUSE", MappedKey::KEY_PLAYPAUSE),
        ("KEY_NUMERIC_0", MappedKey::KEY_KP0),
        ("KEY_NUMERIC_1", MappedKey::KEY_KP1),
        ("KEY_NUMERIC_2", MappedKey::KEY_KP2),
        ("KEY_NUMERIC_3", MappedKey::KEY_KP3),
        ("KEY_NUMERIC_4", MappedKey::KEY_KP4),
        ("KEY_NUMERIC_5", MappedKey::KEY_KP5),
        ("KEY_NUMERIC_6", MappedKey::KEY_KP6),
        ("KEY_NUMERIC_7", MappedKey::KEY_KP7),
        ("KEY_NUMERIC_8", MappedKey::KEY_KP8),
        ("KEY_NUMERIC_9", MappedKey::KEY_KP9),
    ];

    pub struct ConfigHolder {
        pub keys: Vec<MappedKey>,
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
