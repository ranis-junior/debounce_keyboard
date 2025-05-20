pub mod debounce {
    use crate::device::windows::config::ConfigHolder;
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, SystemTime};
    use strum::{EnumIter, IntoEnumIterator};

    use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        DispatchMessageA, GetMessageA, MSG, TranslateMessage,
    };
    use windows::core::Error;

    use windows::Win32::{UI::Input::KeyboardAndMouse::*, UI::WindowsAndMessaging::*};

    static KEY_EVENT_HOLDER: OnceLock<Mutex<KeyEventHolder>> = OnceLock::new();
    static CONFIG_HOLDER: OnceLock<ConfigHolder> = OnceLock::new();

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
        KEY_MULTIPLY,
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
        KEY_KP4,
        KEY_KP5,
        KEY_KP6,
        KEY_KP1,
        KEY_KP2,
        KEY_KP3,
        KEY_KP0,
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
        KEY_OTHER(u16),
    }

    impl From<u16> for MappedKey {
        // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
        fn from(code: u16) -> MappedKey {
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
                0x6A => MappedKey::KEY_MULTIPLY,
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
        container: HashMap<u16, KeyEvent>,
    }

    impl KeyEventHolder {
        pub fn new(minimum_delay: u64) -> KeyEventHolder {
            KeyEventHolder {
                minimum_delay: Duration::from_millis(minimum_delay),
                container: HashMap::new(),
            }
        }

        fn insert_event(&mut self, key_code: u16, key_event: KeyEvent) {
            self.container.insert(key_code, key_event);
        }

        fn remove_event(&mut self, key_code: u16) {
            self.container.remove(&key_code);
        }

        fn last_timestamp(&mut self, key_code: u16) -> Option<&mut KeyEvent> {
            self.container.get_mut(&key_code)
        }
    }

    #[derive(Debug)]
    pub struct KeyEvent {
        pub keycode: u16,
        pub value: u32,
        pub timestamp: Duration,
        pub valid: bool,
    }

    impl KeyEvent {
        pub fn new(keycode: u16, value: u32, timestamp: Duration, valid: bool) -> KeyEvent {
            KeyEvent {
                keycode,
                value,
                timestamp,
                valid,
            }
        }
    }

    fn should_skip(
        key_code: u16,
        key_value: u32,
        timestamp: u32,
        key_holder: &mut KeyEventHolder,
        config_holder: &ConfigHolder,
    ) -> bool {
        /*
           @TODO find a way to verify key press event
        */
        if let Some(key_event   ) = key_holder.container.get(&key_code) {
            if key_value == key_event.value
                && (key_value == WM_KEYDOWN || key_value == WM_SYSKEYDOWN)
                && key_event.valid
            {
                return false;
            }
        }

        if !config_holder.keys.contains(&MappedKey::from(key_code)) {
            return false;
        }
        let minimum_delay = key_holder.minimum_delay;
        match key_holder.last_timestamp(key_code) {
            Some(key_event) => {
                let last_timestamp = Duration::from_millis(timestamp as u64);
                let previous_timestamp = key_event.timestamp;

                let time_expired =
                    last_timestamp.saturating_sub(previous_timestamp) > minimum_delay;

                if key_value == WM_KEYDOWN || key_value == WM_SYSKEYDOWN {
                    if time_expired {
                        key_event.valid = true;
                        key_holder.insert_event(
                            key_code,
                            KeyEvent::new(
                                key_code,
                                key_value,
                                Duration::from_millis(timestamp as u64),
                                true,
                            ),
                        );
                        return false;
                    }
                    return true;
                } else if key_event.valid {
                    key_event.valid = false;
                    return false;
                } else {
                    return true;
                }
            }
            None => {
                if key_value == WM_KEYDOWN || key_value == WM_SYSKEYDOWN {
                    key_holder.insert_event(
                        key_code,
                        KeyEvent::new(
                            key_code,
                            key_value,
                            Duration::from_millis(timestamp as u64),
                            true,
                        ),
                    );
                }
                false
            }
        }
    }

    pub fn run_message_loop() {
        let mut msg: MSG = MSG::default();
        while unsafe { GetMessageA(&mut msg, Some(HWND::default()), 0, 0) } == true {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }

    unsafe extern "system" fn keyboard_proc(
        n_code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        if n_code == HC_ACTION as i32 {
            let kbd_struct = &*(l_param.0 as *const KBDLLHOOKSTRUCT);
            let vk_code = kbd_struct.vkCode;

            if vk_code == VK_F8.0 as u32 || vk_code == VK_F9.0 as u32 {
                CallNextHookEx(Some(HHOOK::default()), n_code, w_param, l_param);
            }

            let mut key_event_holder = KEY_EVENT_HOLDER.get().unwrap().lock().unwrap();
            let config_holder = CONFIG_HOLDER.get().unwrap();

            let skip = should_skip(
                kbd_struct.vkCode as u16,
                w_param.0 as u32,
                kbd_struct.time,
                &mut key_event_holder,
                &config_holder,
            );

            if skip {
                return LRESULT(1);
            }
        }

        CallNextHookEx(Some(HHOOK::default()), n_code, w_param, l_param)
    }

    pub fn setup_windows_ll_keyboard_hook(
        key_event_holder: KeyEventHolder,
        config_holder: ConfigHolder,
    ) -> Result<(), Error> {
        unsafe {
            KEY_EVENT_HOLDER.get_or_init(|| Mutex::new(key_event_holder));
            CONFIG_HOLDER.get_or_init(|| config_holder);

            let h_instance = HINSTANCE::default();
            let mut hook =
                SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_proc), Some(h_instance), 0)
                    .unwrap();

            if hook.is_invalid() {
                panic!("Erro ao instalar o hook.");
            }

            println!("Hook instalado. Pressione Ctrl+C para sair.");
            Ok(())
        }
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
    }
}

pub mod config {
    use crate::device::windows::debounce::MappedKey;
    use crate::device::windows::debounce::get_all_keys_code;
    use config::{Config, File, FileFormat};
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::LazyLock;
    use strum::IntoEnumIterator;
    static AVAILABLE_KEYS: LazyLock<Vec<(String, MappedKey)>> = LazyLock::new(|| {
        let mut keys: Vec<(String, MappedKey)> = Vec::new();
        for key in MappedKey::iter() {
            keys.push((format!("{key:?}"), key));
        }
        keys
    });

    pub struct ConfigHolder {
        pub keys: Vec<MappedKey>,
        pub delay_ms: u64,
    }

    impl Display for ConfigHolder {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let keys = AVAILABLE_KEYS
                .iter()
                .filter(|(_, v)| self.keys.contains(v))
                .map(|(k, _)| k.to_owned())
                .collect::<Vec<_>>()
                .join(",");
            write!(f, "keys={keys}\ndelay_ms={}", self.delay_ms)
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

        let delay_ms = match settings.get("delay_ms") {
            Some(value) => value.parse::<u64>().unwrap(),
            None => 85,
        };

        ConfigHolder { keys, delay_ms }
    }

    pub fn save_config_to_path(path: &PathBuf, config: &ConfigHolder) {
        let mut config_file = std::fs::File::create(path).expect("Failed to create config file");
        config_file
            .write_all(config.to_string().as_bytes())
            .expect("Failed to write to config file");
    }

    fn get_keys_code(keys: &str) -> Vec<MappedKey> {
        let available_keys_map = AVAILABLE_KEYS
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<HashMap<String, MappedKey>>();
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
