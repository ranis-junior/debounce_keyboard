pub mod debounce {
    use crate::device::windows::config::ConfigHolder;
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};
    use strum::EnumIter;

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
    pub enum KeybdKey {
        BackspaceKey,
        TabKey,
        EnterKey,
        EscapeKey,
        SpaceKey,
        PageUpKey,
        PageDownKey,
        EndKey,
        HomeKey,
        LeftKey,
        UpKey,
        RightKey,
        DownKey,
        InsertKey,
        DeleteKey,
        Numrow0Key,
        Numrow1Key,
        Numrow2Key,
        Numrow3Key,
        Numrow4Key,
        Numrow5Key,
        Numrow6Key,
        Numrow7Key,
        Numrow8Key,
        Numrow9Key,
        AKey,
        BKey,
        CKey,
        DKey,
        EKey,
        FKey,
        GKey,
        HKey,
        IKey,
        JKey,
        KKey,
        LKey,
        MKey,
        NKey,
        OKey,
        PKey,
        QKey,
        RKey,
        SKey,
        TKey,
        UKey,
        VKey,
        WKey,
        XKey,
        YKey,
        ZKey,
        LSuper,
        RSuper,
        Numpad0Key,
        Numpad1Key,
        Numpad2Key,
        Numpad3Key,
        Numpad4Key,
        Numpad5Key,
        Numpad6Key,
        Numpad7Key,
        Numpad8Key,
        Numpad9Key,
        F1Key,
        F2Key,
        F3Key,
        F4Key,
        F5Key,
        F6Key,
        F7Key,
        F8Key,
        F9Key,
        F10Key,
        F11Key,
        F12Key,
        F13Key,
        F14Key,
        F15Key,
        F16Key,
        F17Key,
        F18Key,
        F19Key,
        F20Key,
        F21Key,
        F22Key,
        F23Key,
        F24Key,
        NumLockKey,
        ScrollLockKey,
        CapsLockKey,
        LShiftKey,
        RShiftKey,
        LControlKey,
        RControlKey,
        LAltKey,
        RAltKey,

        BrowserBackKey,
        BrowserForwardKey,
        BrowserRefreshKey,

        VolumeMuteKey,
        VolumeDownKey,
        VolumeUpKey,

        MediaNextTrackKey,
        MediaPrevTrackKey,
        MediaStopKey,
        MediaPlayPauseKey,

        BackquoteKey,
        SlashKey,
        BackslashKey,
        CommaKey,
        PeriodKey,
        MinusKey,
        QuoteKey,
        SemicolonKey,
        LBracketKey,
        RBracketKey,
        EqualKey,

        #[strum(disabled)]
        OtherKey(u64),
    }

    impl From<u64> for KeybdKey {
        // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
        fn from(code: u64) -> KeybdKey {
            match code {
                0x08 => BackspaceKey,
                0x09 => TabKey,
                0x0D => EnterKey,
                0x1B => EscapeKey,
                0x20 => SpaceKey,
                0x21 => PageUpKey,
                0x22 => PageDownKey,
                0x23 => EndKey,
                0x24 => HomeKey,
                0x25 => LeftKey,
                0x26 => UpKey,
                0x27 => RightKey,
                0x28 => DownKey,
                0x2D => InsertKey,
                0x2E => DeleteKey,
                0x30 => Numrow0Key,
                0x31 => Numrow1Key,
                0x32 => Numrow2Key,
                0x33 => Numrow3Key,
                0x34 => Numrow4Key,
                0x35 => Numrow5Key,
                0x36 => Numrow6Key,
                0x37 => Numrow7Key,
                0x38 => Numrow8Key,
                0x39 => Numrow9Key,
                0x41 => AKey,
                0x42 => BKey,
                0x43 => CKey,
                0x44 => DKey,
                0x45 => EKey,
                0x46 => FKey,
                0x47 => GKey,
                0x48 => HKey,
                0x49 => IKey,
                0x4A => JKey,
                0x4B => KKey,
                0x4C => LKey,
                0x4D => MKey,
                0x4E => NKey,
                0x4F => OKey,
                0x50 => PKey,
                0x51 => QKey,
                0x52 => RKey,
                0x53 => SKey,
                0x54 => TKey,
                0x55 => UKey,
                0x56 => VKey,
                0x57 => WKey,
                0x58 => XKey,
                0x59 => YKey,
                0x5A => ZKey,
                0x5B => LSuper,
                0x5C => RSuper,
                0x60 => Numpad0Key,
                0x61 => Numpad1Key,
                0x62 => Numpad2Key,
                0x63 => Numpad3Key,
                0x64 => Numpad4Key,
                0x65 => Numpad5Key,
                0x66 => Numpad6Key,
                0x67 => Numpad7Key,
                0x68 => Numpad8Key,
                0x69 => Numpad9Key,
                0x70 => F1Key,
                0x71 => F2Key,
                0x72 => F3Key,
                0x73 => F4Key,
                0x74 => F5Key,
                0x75 => F6Key,
                0x76 => F7Key,
                0x77 => F8Key,
                0x78 => F9Key,
                0x79 => F10Key,
                0x7A => F11Key,
                0x7B => F12Key,
                0x7C => F13Key,
                0x7D => F14Key,
                0x7E => F15Key,
                0x7F => F16Key,
                0x80 => F17Key,
                0x81 => F18Key,
                0x82 => F19Key,
                0x83 => F20Key,
                0x84 => F21Key,
                0x85 => F22Key,
                0x86 => F23Key,
                0x87 => F24Key,
                0x90 => NumLockKey,
                0x91 => ScrollLockKey,
                0x14 => CapsLockKey,
                0xA0 => LShiftKey,
                0xA1 => RShiftKey,
                0xA2 => LControlKey,
                0xA3 => RControlKey,
                0xA4 => LAltKey,
                0xA5 => RAltKey,
                0xA6 => BrowserBackKey,
                0xA7 => BrowserForwardKey,
                0xA8 => BrowserRefreshKey,
                0xAD => VolumeMuteKey,
                0xAE => VolumeDownKey,
                0xAF => VolumeUpKey,
                0xB0 => MediaNextTrackKey,
                0xB1 => MediaPrevTrackKey,
                0xB2 => MediaStopKey,
                0xB3 => MediaPlayPauseKey,
                0xC0 => BackquoteKey,
                0xBF => SlashKey,
                0xDC => BackslashKey,
                0xBC => CommaKey,
                0xBE => PeriodKey,
                0xBD => MinusKey,
                0xDE => QuoteKey,
                0xBA => SemicolonKey,
                0xDB => LBracketKey,
                0xDD => RBracketKey,
                0xBB => EqualKey,
                _ => OtherKey(code),
            }
        }
    }

    impl std::fmt::Display for KeybdKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    KeybdKey::BackspaceKey => "Backspace",
                    KeybdKey::TabKey => "Tab",
                    KeybdKey::EnterKey => "Enter",
                    KeybdKey::EscapeKey => "Escape",
                    KeybdKey::SpaceKey => "Space",
                    KeybdKey::PageUpKey => "PageUp",
                    KeybdKey::PageDownKey => "PageDown",
                    KeybdKey::EndKey => "End",
                    KeybdKey::HomeKey => "Home",
                    KeybdKey::LeftKey => "Left",
                    KeybdKey::UpKey => "Up",
                    KeybdKey::RightKey => "Right",
                    KeybdKey::DownKey => "Down",
                    KeybdKey::InsertKey => "Insert",
                    KeybdKey::DeleteKey => "Delete",
                    KeybdKey::Numrow0Key => "0",
                    KeybdKey::Numrow1Key => "1",
                    KeybdKey::Numrow2Key => "2",
                    KeybdKey::Numrow3Key => "3",
                    KeybdKey::Numrow4Key => "4",
                    KeybdKey::Numrow5Key => "5",
                    KeybdKey::Numrow6Key => "6",
                    KeybdKey::Numrow7Key => "7",
                    KeybdKey::Numrow8Key => "8",
                    KeybdKey::Numrow9Key => "9",
                    KeybdKey::AKey => "a",
                    KeybdKey::BKey => "b",
                    KeybdKey::CKey => "c",
                    KeybdKey::DKey => "d",
                    KeybdKey::EKey => "e",
                    KeybdKey::FKey => "f",
                    KeybdKey::GKey => "g",
                    KeybdKey::HKey => "h",
                    KeybdKey::IKey => "i",
                    KeybdKey::JKey => "j",
                    KeybdKey::KKey => "k",
                    KeybdKey::LKey => "l",
                    KeybdKey::MKey => "m",
                    KeybdKey::NKey => "n",
                    KeybdKey::OKey => "o",
                    KeybdKey::PKey => "p",
                    KeybdKey::QKey => "q",
                    KeybdKey::RKey => "r",
                    KeybdKey::SKey => "s",
                    KeybdKey::TKey => "t",
                    KeybdKey::UKey => "u",
                    KeybdKey::VKey => "v",
                    KeybdKey::WKey => "w",
                    KeybdKey::XKey => "x",
                    KeybdKey::YKey => "y",
                    KeybdKey::ZKey => "z",
                    KeybdKey::LSuper => "LeftWindows",
                    KeybdKey::RSuper => "RightWindows",
                    KeybdKey::Numpad0Key => "NumPad0",
                    KeybdKey::Numpad1Key => "NumPad1",
                    KeybdKey::Numpad2Key => "NumPad2",
                    KeybdKey::Numpad3Key => "NumPad3",
                    KeybdKey::Numpad4Key => "NumPad4",
                    KeybdKey::Numpad5Key => "NumPad5",
                    KeybdKey::Numpad6Key => "NumPad6",
                    KeybdKey::Numpad7Key => "NumPad7",
                    KeybdKey::Numpad8Key => "NumPad8",
                    KeybdKey::Numpad9Key => "NumPad9",
                    KeybdKey::F1Key => "F1",
                    KeybdKey::F2Key => "F2",
                    KeybdKey::F3Key => "F3",
                    KeybdKey::F4Key => "F4",
                    KeybdKey::F5Key => "F5",
                    KeybdKey::F6Key => "F6",
                    KeybdKey::F7Key => "F7",
                    KeybdKey::F8Key => "F8",
                    KeybdKey::F9Key => "F9",
                    KeybdKey::F10Key => "F10",
                    KeybdKey::F11Key => "F11",
                    KeybdKey::F12Key => "F12",
                    KeybdKey::F13Key => "F13",
                    KeybdKey::F14Key => "F14",
                    KeybdKey::F15Key => "F15",
                    KeybdKey::F16Key => "F16",
                    KeybdKey::F17Key => "F17",
                    KeybdKey::F18Key => "F18",
                    KeybdKey::F19Key => "F19",
                    KeybdKey::F20Key => "F20",
                    KeybdKey::F21Key => "F21",
                    KeybdKey::F22Key => "F22",
                    KeybdKey::F23Key => "F23",
                    KeybdKey::F24Key => "F24",
                    KeybdKey::NumLockKey => "NumLock",
                    KeybdKey::ScrollLockKey => "ScrollLock",
                    KeybdKey::CapsLockKey => "CapsLock",
                    KeybdKey::LShiftKey => "LeftShift",
                    KeybdKey::RShiftKey => "RightShift",
                    KeybdKey::LControlKey => "LeftControl",
                    KeybdKey::RControlKey => "RightControl",
                    KeybdKey::LAltKey => "LeftAlt",
                    KeybdKey::RAltKey => "RightAlt",
                    KeybdKey::BrowserBackKey => "Back",
                    KeybdKey::BrowserForwardKey => "Forward",
                    KeybdKey::BrowserRefreshKey => "Refresh",
                    KeybdKey::VolumeMuteKey => "VolumeMute",
                    KeybdKey::VolumeDownKey => "VolumeDown",
                    KeybdKey::VolumeUpKey => "VolumeUp",
                    KeybdKey::MediaNextTrackKey => "MediaNext",
                    KeybdKey::MediaPrevTrackKey => "MediaPrevious",
                    KeybdKey::MediaStopKey => "MediaStop",
                    KeybdKey::MediaPlayPauseKey => "MediaPlay",
                    KeybdKey::BackquoteKey => "Backquote",
                    KeybdKey::SlashKey => "Slash",
                    KeybdKey::BackslashKey => "Backslash",
                    KeybdKey::CommaKey => "Comma",
                    KeybdKey::PeriodKey => "Period",
                    KeybdKey::MinusKey => "Minus",
                    KeybdKey::QuoteKey => "QuoteKey",
                    KeybdKey::SemicolonKey => "Semicolon",
                    KeybdKey::LBracketKey => "LeftBracket",
                    KeybdKey::RBracketKey => "RightBracket",
                    KeybdKey::EqualKey => "Equal",
                    KeybdKey::OtherKey(code) => return write!(f, "OtherKey({code})"),
                }
            )
        }
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
    use crate::device::debounce::KeybdKey;
    use crate::device::windows::debounce::get_all_keys_code;
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
