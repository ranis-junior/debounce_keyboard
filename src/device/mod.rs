#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod debounce {
    use crate::device::config::ConfigHolder;
    use evdev::uinput::VirtualDevice;
    use evdev::{AttributeSet, Device, FetchEventsSynced, KeyCode, KeyEvent};
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};

    const ALL_KEYS: [KeyCode; 244] = [
        KeyCode::KEY_RESERVED,
        KeyCode::KEY_ESC,
        KeyCode::KEY_1,
        KeyCode::KEY_2,
        KeyCode::KEY_3,
        KeyCode::KEY_4,
        KeyCode::KEY_5,
        KeyCode::KEY_6,
        KeyCode::KEY_7,
        KeyCode::KEY_8,
        KeyCode::KEY_9,
        KeyCode::KEY_0,
        KeyCode::KEY_MINUS,
        KeyCode::KEY_EQUAL,
        KeyCode::KEY_BACKSPACE,
        KeyCode::KEY_TAB,
        KeyCode::KEY_Q,
        KeyCode::KEY_W,
        KeyCode::KEY_E,
        KeyCode::KEY_R,
        KeyCode::KEY_T,
        KeyCode::KEY_Y,
        KeyCode::KEY_U,
        KeyCode::KEY_I,
        KeyCode::KEY_O,
        KeyCode::KEY_P,
        KeyCode::KEY_LEFTBRACE,
        KeyCode::KEY_RIGHTBRACE,
        KeyCode::KEY_ENTER,
        KeyCode::KEY_LEFTCTRL,
        KeyCode::KEY_A,
        KeyCode::KEY_S,
        KeyCode::KEY_D,
        KeyCode::KEY_F,
        KeyCode::KEY_G,
        KeyCode::KEY_H,
        KeyCode::KEY_J,
        KeyCode::KEY_K,
        KeyCode::KEY_L,
        KeyCode::KEY_SEMICOLON,
        KeyCode::KEY_APOSTROPHE,
        KeyCode::KEY_GRAVE,
        KeyCode::KEY_LEFTSHIFT,
        KeyCode::KEY_BACKSLASH,
        KeyCode::KEY_Z,
        KeyCode::KEY_X,
        KeyCode::KEY_C,
        KeyCode::KEY_V,
        KeyCode::KEY_B,
        KeyCode::KEY_N,
        KeyCode::KEY_M,
        KeyCode::KEY_COMMA,
        KeyCode::KEY_DOT,
        KeyCode::KEY_SLASH,
        KeyCode::KEY_RIGHTSHIFT,
        KeyCode::KEY_KPASTERISK,
        KeyCode::KEY_LEFTALT,
        KeyCode::KEY_SPACE,
        KeyCode::KEY_CAPSLOCK,
        KeyCode::KEY_F1,
        KeyCode::KEY_F2,
        KeyCode::KEY_F3,
        KeyCode::KEY_F4,
        KeyCode::KEY_F5,
        KeyCode::KEY_F6,
        KeyCode::KEY_F7,
        KeyCode::KEY_F8,
        KeyCode::KEY_F9,
        KeyCode::KEY_F10,
        KeyCode::KEY_NUMLOCK,
        KeyCode::KEY_SCROLLLOCK,
        KeyCode::KEY_KP7,
        KeyCode::KEY_KP8,
        KeyCode::KEY_KP9,
        KeyCode::KEY_KPMINUS,
        KeyCode::KEY_KP4,
        KeyCode::KEY_KP5,
        KeyCode::KEY_KP6,
        KeyCode::KEY_KPPLUS,
        KeyCode::KEY_KP1,
        KeyCode::KEY_KP2,
        KeyCode::KEY_KP3,
        KeyCode::KEY_KP0,
        KeyCode::KEY_KPDOT,
        KeyCode::KEY_ZENKAKUHANKAKU,
        KeyCode::KEY_102ND,
        KeyCode::KEY_F11,
        KeyCode::KEY_F12,
        KeyCode::KEY_RO,
        KeyCode::KEY_KATAKANA,
        KeyCode::KEY_HIRAGANA,
        KeyCode::KEY_HENKAN,
        KeyCode::KEY_KATAKANAHIRAGANA,
        KeyCode::KEY_MUHENKAN,
        KeyCode::KEY_KPJPCOMMA,
        KeyCode::KEY_KPENTER,
        KeyCode::KEY_RIGHTCTRL,
        KeyCode::KEY_KPSLASH,
        KeyCode::KEY_SYSRQ,
        KeyCode::KEY_RIGHTALT,
        KeyCode::KEY_LINEFEED,
        KeyCode::KEY_HOME,
        KeyCode::KEY_UP,
        KeyCode::KEY_PAGEUP,
        KeyCode::KEY_LEFT,
        KeyCode::KEY_RIGHT,
        KeyCode::KEY_END,
        KeyCode::KEY_DOWN,
        KeyCode::KEY_PAGEDOWN,
        KeyCode::KEY_INSERT,
        KeyCode::KEY_DELETE,
        KeyCode::KEY_MACRO,
        KeyCode::KEY_MUTE,
        KeyCode::KEY_VOLUMEDOWN,
        KeyCode::KEY_VOLUMEUP,
        KeyCode::KEY_POWER,
        KeyCode::KEY_KPEQUAL,
        KeyCode::KEY_KPPLUSMINUS,
        KeyCode::KEY_PAUSE,
        KeyCode::KEY_SCALE,
        KeyCode::KEY_KPCOMMA,
        KeyCode::KEY_HANGEUL,
        KeyCode::KEY_HANJA,
        KeyCode::KEY_YEN,
        KeyCode::KEY_LEFTMETA,
        KeyCode::KEY_RIGHTMETA,
        KeyCode::KEY_COMPOSE,
        KeyCode::KEY_STOP,
        KeyCode::KEY_AGAIN,
        KeyCode::KEY_PROPS,
        KeyCode::KEY_UNDO,
        KeyCode::KEY_FRONT,
        KeyCode::KEY_COPY,
        KeyCode::KEY_OPEN,
        KeyCode::KEY_PASTE,
        KeyCode::KEY_FIND,
        KeyCode::KEY_CUT,
        KeyCode::KEY_HELP,
        KeyCode::KEY_MENU,
        KeyCode::KEY_CALC,
        KeyCode::KEY_SETUP,
        KeyCode::KEY_SLEEP,
        KeyCode::KEY_WAKEUP,
        KeyCode::KEY_FILE,
        KeyCode::KEY_SENDFILE,
        KeyCode::KEY_DELETEFILE,
        KeyCode::KEY_XFER,
        KeyCode::KEY_PROG1,
        KeyCode::KEY_PROG2,
        KeyCode::KEY_WWW,
        KeyCode::KEY_MSDOS,
        KeyCode::KEY_COFFEE,
        KeyCode::KEY_DIRECTION,
        KeyCode::KEY_ROTATE_DISPLAY,
        KeyCode::KEY_CYCLEWINDOWS,
        KeyCode::KEY_MAIL,
        KeyCode::KEY_BOOKMARKS,
        KeyCode::KEY_COMPUTER,
        KeyCode::KEY_BACK,
        KeyCode::KEY_FORWARD,
        KeyCode::KEY_CLOSECD,
        KeyCode::KEY_EJECTCD,
        KeyCode::KEY_EJECTCLOSECD,
        KeyCode::KEY_NEXTSONG,
        KeyCode::KEY_PLAYPAUSE,
        KeyCode::KEY_PREVIOUSSONG,
        KeyCode::KEY_STOPCD,
        KeyCode::KEY_RECORD,
        KeyCode::KEY_REWIND,
        KeyCode::KEY_PHONE,
        KeyCode::KEY_ISO,
        KeyCode::KEY_CONFIG,
        KeyCode::KEY_HOMEPAGE,
        KeyCode::KEY_REFRESH,
        KeyCode::KEY_EXIT,
        KeyCode::KEY_MOVE,
        KeyCode::KEY_EDIT,
        KeyCode::KEY_SCROLLUP,
        KeyCode::KEY_SCROLLDOWN,
        KeyCode::KEY_KPLEFTPAREN,
        KeyCode::KEY_KPRIGHTPAREN,
        KeyCode::KEY_NEW,
        KeyCode::KEY_REDO,
        KeyCode::KEY_F13,
        KeyCode::KEY_F14,
        KeyCode::KEY_F15,
        KeyCode::KEY_F16,
        KeyCode::KEY_F17,
        KeyCode::KEY_F18,
        KeyCode::KEY_F19,
        KeyCode::KEY_F20,
        KeyCode::KEY_F21,
        KeyCode::KEY_F22,
        KeyCode::KEY_F23,
        KeyCode::KEY_F24,
        KeyCode::KEY_PLAYCD,
        KeyCode::KEY_PAUSECD,
        KeyCode::KEY_PROG3,
        KeyCode::KEY_PROG4,
        KeyCode::KEY_DASHBOARD,
        KeyCode::KEY_SUSPEND,
        KeyCode::KEY_CLOSE,
        KeyCode::KEY_PLAY,
        KeyCode::KEY_FASTFORWARD,
        KeyCode::KEY_BASSBOOST,
        KeyCode::KEY_PRINT,
        KeyCode::KEY_HP,
        KeyCode::KEY_CAMERA,
        KeyCode::KEY_SOUND,
        KeyCode::KEY_QUESTION,
        KeyCode::KEY_EMAIL,
        KeyCode::KEY_CHAT,
        KeyCode::KEY_SEARCH,
        KeyCode::KEY_CONNECT,
        KeyCode::KEY_FINANCE,
        KeyCode::KEY_SPORT,
        KeyCode::KEY_SHOP,
        KeyCode::KEY_ALTERASE,
        KeyCode::KEY_CANCEL,
        KeyCode::KEY_BRIGHTNESSDOWN,
        KeyCode::KEY_BRIGHTNESSUP,
        KeyCode::KEY_MEDIA,
        KeyCode::KEY_SWITCHVIDEOMODE,
        KeyCode::KEY_KBDILLUMTOGGLE,
        KeyCode::KEY_KBDILLUMDOWN,
        KeyCode::KEY_KBDILLUMUP,
        KeyCode::KEY_SEND,
        KeyCode::KEY_REPLY,
        KeyCode::KEY_FORWARDMAIL,
        KeyCode::KEY_SAVE,
        KeyCode::KEY_DOCUMENTS,
        KeyCode::KEY_BATTERY,
        KeyCode::KEY_BLUETOOTH,
        KeyCode::KEY_WLAN,
        KeyCode::KEY_UWB,
        KeyCode::KEY_UNKNOWN,
        KeyCode::KEY_VIDEO_NEXT,
        KeyCode::KEY_VIDEO_PREV,
        KeyCode::KEY_BRIGHTNESS_CYCLE,
        KeyCode::KEY_BRIGHTNESS_AUTO,
        KeyCode::KEY_DISPLAY_OFF,
        KeyCode::KEY_WWAN,
        KeyCode::KEY_RFKILL,
        KeyCode::KEY_MICMUTE,
    ];

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
    use crate::device::debounce::get_all_keys_code;
    use config::{Config, File, FileFormat};
    use evdev::{AttributeSet, KeyCode};
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::io::Write;
    use std::path::PathBuf;

    const AVAILABLE_KEYS: [(&str, KeyCode); 95] = [
        ("KEY_ESC", KeyCode::KEY_ESC),
        ("KEY_1", KeyCode::KEY_1),
        ("KEY_2", KeyCode::KEY_2),
        ("KEY_3", KeyCode::KEY_3),
        ("KEY_4", KeyCode::KEY_4),
        ("KEY_5", KeyCode::KEY_5),
        ("KEY_6", KeyCode::KEY_6),
        ("KEY_7", KeyCode::KEY_7),
        ("KEY_8", KeyCode::KEY_8),
        ("KEY_9", KeyCode::KEY_9),
        ("KEY_0", KeyCode::KEY_0),
        ("KEY_MINUS", KeyCode::KEY_MINUS),
        ("KEY_EQUAL", KeyCode::KEY_EQUAL),
        ("KEY_BACKSPACE", KeyCode::KEY_BACKSPACE),
        ("KEY_TAB", KeyCode::KEY_TAB),
        ("KEY_Q", KeyCode::KEY_Q),
        ("KEY_W", KeyCode::KEY_W),
        ("KEY_E", KeyCode::KEY_E),
        ("KEY_R", KeyCode::KEY_R),
        ("KEY_T", KeyCode::KEY_T),
        ("KEY_Y", KeyCode::KEY_Y),
        ("KEY_U", KeyCode::KEY_U),
        ("KEY_I", KeyCode::KEY_I),
        ("KEY_O", KeyCode::KEY_O),
        ("KEY_P", KeyCode::KEY_P),
        ("KEY_LEFTBRACE", KeyCode::KEY_LEFTBRACE),
        ("KEY_RIGHTBRACE", KeyCode::KEY_RIGHTBRACE),
        ("KEY_ENTER", KeyCode::KEY_ENTER),
        ("KEY_LEFTCTRL", KeyCode::KEY_LEFTCTRL),
        ("KEY_A", KeyCode::KEY_A),
        ("KEY_S", KeyCode::KEY_S),
        ("KEY_D", KeyCode::KEY_D),
        ("KEY_F", KeyCode::KEY_F),
        ("KEY_G", KeyCode::KEY_G),
        ("KEY_H", KeyCode::KEY_H),
        ("KEY_J", KeyCode::KEY_J),
        ("KEY_K", KeyCode::KEY_K),
        ("KEY_L", KeyCode::KEY_L),
        ("KEY_SEMICOLON", KeyCode::KEY_SEMICOLON),
        ("KEY_APOSTROPHE", KeyCode::KEY_APOSTROPHE),
        ("KEY_GRAVE", KeyCode::KEY_GRAVE),
        ("KEY_LEFTSHIFT", KeyCode::KEY_LEFTSHIFT),
        ("KEY_BACKSLASH", KeyCode::KEY_BACKSLASH),
        ("KEY_Z", KeyCode::KEY_Z),
        ("KEY_X", KeyCode::KEY_X),
        ("KEY_C", KeyCode::KEY_C),
        ("KEY_V", KeyCode::KEY_V),
        ("KEY_B", KeyCode::KEY_B),
        ("KEY_N", KeyCode::KEY_N),
        ("KEY_M", KeyCode::KEY_M),
        ("KEY_COMMA", KeyCode::KEY_COMMA),
        ("KEY_DOT", KeyCode::KEY_DOT),
        ("KEY_SLASH", KeyCode::KEY_SLASH),
        ("KEY_RIGHTSHIFT", KeyCode::KEY_RIGHTSHIFT),
        ("KEY_KPASTERISK", KeyCode::KEY_KPASTERISK),
        ("KEY_LEFTALT", KeyCode::KEY_LEFTALT),
        ("KEY_SPACE", KeyCode::KEY_SPACE),
        ("KEY_CAPSLOCK", KeyCode::KEY_CAPSLOCK),
        ("KEY_F1", KeyCode::KEY_F1),
        ("KEY_F2", KeyCode::KEY_F2),
        ("KEY_F3", KeyCode::KEY_F3),
        ("KEY_F4", KeyCode::KEY_F4),
        ("KEY_F5", KeyCode::KEY_F5),
        ("KEY_F6", KeyCode::KEY_F6),
        ("KEY_F7", KeyCode::KEY_F7),
        ("KEY_F8", KeyCode::KEY_F8),
        ("KEY_F9", KeyCode::KEY_F9),
        ("KEY_F10", KeyCode::KEY_F10),
        ("KEY_NUMLOCK", KeyCode::KEY_NUMLOCK),
        ("KEY_SCROLLLOCK", KeyCode::KEY_SCROLLLOCK),
        ("KEY_F11", KeyCode::KEY_F11),
        ("KEY_F12", KeyCode::KEY_F12),
        ("KEY_SYSRQ", KeyCode::KEY_SYSRQ),
        ("KEY_RIGHTALT", KeyCode::KEY_RIGHTALT),
        ("KEY_HOME", KeyCode::KEY_HOME),
        ("KEY_UP", KeyCode::KEY_UP),
        ("KEY_PAGEUP", KeyCode::KEY_PAGEUP),
        ("KEY_LEFT", KeyCode::KEY_LEFT),
        ("KEY_RIGHT", KeyCode::KEY_RIGHT),
        ("KEY_END", KeyCode::KEY_END),
        ("KEY_DOWN", KeyCode::KEY_DOWN),
        ("KEY_PAGEDOWN", KeyCode::KEY_PAGEDOWN),
        ("KEY_INSERT", KeyCode::KEY_INSERT),
        ("KEY_DELETE", KeyCode::KEY_DELETE),
        ("KEY_PAUSE", KeyCode::KEY_PAUSE),
        ("KEY_NUMERIC_0", KeyCode::KEY_NUMERIC_0),
        ("KEY_NUMERIC_1", KeyCode::KEY_NUMERIC_1),
        ("KEY_NUMERIC_2", KeyCode::KEY_NUMERIC_2),
        ("KEY_NUMERIC_3", KeyCode::KEY_NUMERIC_3),
        ("KEY_NUMERIC_4", KeyCode::KEY_NUMERIC_4),
        ("KEY_NUMERIC_5", KeyCode::KEY_NUMERIC_5),
        ("KEY_NUMERIC_6", KeyCode::KEY_NUMERIC_6),
        ("KEY_NUMERIC_7", KeyCode::KEY_NUMERIC_7),
        ("KEY_NUMERIC_8", KeyCode::KEY_NUMERIC_8),
        ("KEY_NUMERIC_9", KeyCode::KEY_NUMERIC_9),
    ];

    pub struct ConfigHolder {
        pub keys: AttributeSet<KeyCode>,
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
