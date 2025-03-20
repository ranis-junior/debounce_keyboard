pub mod debounce {
    use crate::device::linux::config::ConfigHolder;
    use evdev::uinput::VirtualDevice;
    use evdev::{
        AttributeSet, Device as DeviceEvDev, EventSummary, FetchEventsSynced, InputEvent, KeyCode,
        KeyEvent as KeyEventEvDev,
    };
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};

    const ALL_KEYS: [u16; 244] = [
        KeyCode::KEY_RESERVED.code(),
        KeyCode::KEY_ESC.code(),
        KeyCode::KEY_1.code(),
        KeyCode::KEY_2.code(),
        KeyCode::KEY_3.code(),
        KeyCode::KEY_4.code(),
        KeyCode::KEY_5.code(),
        KeyCode::KEY_6.code(),
        KeyCode::KEY_7.code(),
        KeyCode::KEY_8.code(),
        KeyCode::KEY_9.code(),
        KeyCode::KEY_0.code(),
        KeyCode::KEY_MINUS.code(),
        KeyCode::KEY_EQUAL.code(),
        KeyCode::KEY_BACKSPACE.code(),
        KeyCode::KEY_TAB.code(),
        KeyCode::KEY_Q.code(),
        KeyCode::KEY_W.code(),
        KeyCode::KEY_E.code(),
        KeyCode::KEY_R.code(),
        KeyCode::KEY_T.code(),
        KeyCode::KEY_Y.code(),
        KeyCode::KEY_U.code(),
        KeyCode::KEY_I.code(),
        KeyCode::KEY_O.code(),
        KeyCode::KEY_P.code(),
        KeyCode::KEY_LEFTBRACE.code(),
        KeyCode::KEY_RIGHTBRACE.code(),
        KeyCode::KEY_ENTER.code(),
        KeyCode::KEY_LEFTCTRL.code(),
        KeyCode::KEY_A.code(),
        KeyCode::KEY_S.code(),
        KeyCode::KEY_D.code(),
        KeyCode::KEY_F.code(),
        KeyCode::KEY_G.code(),
        KeyCode::KEY_H.code(),
        KeyCode::KEY_J.code(),
        KeyCode::KEY_K.code(),
        KeyCode::KEY_L.code(),
        KeyCode::KEY_SEMICOLON.code(),
        KeyCode::KEY_APOSTROPHE.code(),
        KeyCode::KEY_GRAVE.code(),
        KeyCode::KEY_LEFTSHIFT.code(),
        KeyCode::KEY_BACKSLASH.code(),
        KeyCode::KEY_Z.code(),
        KeyCode::KEY_X.code(),
        KeyCode::KEY_C.code(),
        KeyCode::KEY_V.code(),
        KeyCode::KEY_B.code(),
        KeyCode::KEY_N.code(),
        KeyCode::KEY_M.code(),
        KeyCode::KEY_COMMA.code(),
        KeyCode::KEY_DOT.code(),
        KeyCode::KEY_SLASH.code(),
        KeyCode::KEY_RIGHTSHIFT.code(),
        KeyCode::KEY_KPASTERISK.code(),
        KeyCode::KEY_LEFTALT.code(),
        KeyCode::KEY_SPACE.code(),
        KeyCode::KEY_CAPSLOCK.code(),
        KeyCode::KEY_F1.code(),
        KeyCode::KEY_F2.code(),
        KeyCode::KEY_F3.code(),
        KeyCode::KEY_F4.code(),
        KeyCode::KEY_F5.code(),
        KeyCode::KEY_F6.code(),
        KeyCode::KEY_F7.code(),
        KeyCode::KEY_F8.code(),
        KeyCode::KEY_F9.code(),
        KeyCode::KEY_F10.code(),
        KeyCode::KEY_NUMLOCK.code(),
        KeyCode::KEY_SCROLLLOCK.code(),
        KeyCode::KEY_KP7.code(),
        KeyCode::KEY_KP8.code(),
        KeyCode::KEY_KP9.code(),
        KeyCode::KEY_KPMINUS.code(),
        KeyCode::KEY_KP4.code(),
        KeyCode::KEY_KP5.code(),
        KeyCode::KEY_KP6.code(),
        KeyCode::KEY_KPPLUS.code(),
        KeyCode::KEY_KP1.code(),
        KeyCode::KEY_KP2.code(),
        KeyCode::KEY_KP3.code(),
        KeyCode::KEY_KP0.code(),
        KeyCode::KEY_KPDOT.code(),
        KeyCode::KEY_ZENKAKUHANKAKU.code(),
        KeyCode::KEY_102ND.code(),
        KeyCode::KEY_F11.code(),
        KeyCode::KEY_F12.code(),
        KeyCode::KEY_RO.code(),
        KeyCode::KEY_KATAKANA.code(),
        KeyCode::KEY_HIRAGANA.code(),
        KeyCode::KEY_HENKAN.code(),
        KeyCode::KEY_KATAKANAHIRAGANA.code(),
        KeyCode::KEY_MUHENKAN.code(),
        KeyCode::KEY_KPJPCOMMA.code(),
        KeyCode::KEY_KPENTER.code(),
        KeyCode::KEY_RIGHTCTRL.code(),
        KeyCode::KEY_KPSLASH.code(),
        KeyCode::KEY_SYSRQ.code(),
        KeyCode::KEY_RIGHTALT.code(),
        KeyCode::KEY_LINEFEED.code(),
        KeyCode::KEY_HOME.code(),
        KeyCode::KEY_UP.code(),
        KeyCode::KEY_PAGEUP.code(),
        KeyCode::KEY_LEFT.code(),
        KeyCode::KEY_RIGHT.code(),
        KeyCode::KEY_END.code(),
        KeyCode::KEY_DOWN.code(),
        KeyCode::KEY_PAGEDOWN.code(),
        KeyCode::KEY_INSERT.code(),
        KeyCode::KEY_DELETE.code(),
        KeyCode::KEY_MACRO.code(),
        KeyCode::KEY_MUTE.code(),
        KeyCode::KEY_VOLUMEDOWN.code(),
        KeyCode::KEY_VOLUMEUP.code(),
        KeyCode::KEY_POWER.code(),
        KeyCode::KEY_KPEQUAL.code(),
        KeyCode::KEY_KPPLUSMINUS.code(),
        KeyCode::KEY_PAUSE.code(),
        KeyCode::KEY_SCALE.code(),
        KeyCode::KEY_KPCOMMA.code(),
        KeyCode::KEY_HANGEUL.code(),
        KeyCode::KEY_HANJA.code(),
        KeyCode::KEY_YEN.code(),
        KeyCode::KEY_LEFTMETA.code(),
        KeyCode::KEY_RIGHTMETA.code(),
        KeyCode::KEY_COMPOSE.code(),
        KeyCode::KEY_STOP.code(),
        KeyCode::KEY_AGAIN.code(),
        KeyCode::KEY_PROPS.code(),
        KeyCode::KEY_UNDO.code(),
        KeyCode::KEY_FRONT.code(),
        KeyCode::KEY_COPY.code(),
        KeyCode::KEY_OPEN.code(),
        KeyCode::KEY_PASTE.code(),
        KeyCode::KEY_FIND.code(),
        KeyCode::KEY_CUT.code(),
        KeyCode::KEY_HELP.code(),
        KeyCode::KEY_MENU.code(),
        KeyCode::KEY_CALC.code(),
        KeyCode::KEY_SETUP.code(),
        KeyCode::KEY_SLEEP.code(),
        KeyCode::KEY_WAKEUP.code(),
        KeyCode::KEY_FILE.code(),
        KeyCode::KEY_SENDFILE.code(),
        KeyCode::KEY_DELETEFILE.code(),
        KeyCode::KEY_XFER.code(),
        KeyCode::KEY_PROG1.code(),
        KeyCode::KEY_PROG2.code(),
        KeyCode::KEY_WWW.code(),
        KeyCode::KEY_MSDOS.code(),
        KeyCode::KEY_COFFEE.code(),
        KeyCode::KEY_DIRECTION.code(),
        KeyCode::KEY_ROTATE_DISPLAY.code(),
        KeyCode::KEY_CYCLEWINDOWS.code(),
        KeyCode::KEY_MAIL.code(),
        KeyCode::KEY_BOOKMARKS.code(),
        KeyCode::KEY_COMPUTER.code(),
        KeyCode::KEY_BACK.code(),
        KeyCode::KEY_FORWARD.code(),
        KeyCode::KEY_CLOSECD.code(),
        KeyCode::KEY_EJECTCD.code(),
        KeyCode::KEY_EJECTCLOSECD.code(),
        KeyCode::KEY_NEXTSONG.code(),
        KeyCode::KEY_PLAYPAUSE.code(),
        KeyCode::KEY_PREVIOUSSONG.code(),
        KeyCode::KEY_STOPCD.code(),
        KeyCode::KEY_RECORD.code(),
        KeyCode::KEY_REWIND.code(),
        KeyCode::KEY_PHONE.code(),
        KeyCode::KEY_ISO.code(),
        KeyCode::KEY_CONFIG.code(),
        KeyCode::KEY_HOMEPAGE.code(),
        KeyCode::KEY_REFRESH.code(),
        KeyCode::KEY_EXIT.code(),
        KeyCode::KEY_MOVE.code(),
        KeyCode::KEY_EDIT.code(),
        KeyCode::KEY_SCROLLUP.code(),
        KeyCode::KEY_SCROLLDOWN.code(),
        KeyCode::KEY_KPLEFTPAREN.code(),
        KeyCode::KEY_KPRIGHTPAREN.code(),
        KeyCode::KEY_NEW.code(),
        KeyCode::KEY_REDO.code(),
        KeyCode::KEY_F13.code(),
        KeyCode::KEY_F14.code(),
        KeyCode::KEY_F15.code(),
        KeyCode::KEY_F16.code(),
        KeyCode::KEY_F17.code(),
        KeyCode::KEY_F18.code(),
        KeyCode::KEY_F19.code(),
        KeyCode::KEY_F20.code(),
        KeyCode::KEY_F21.code(),
        KeyCode::KEY_F22.code(),
        KeyCode::KEY_F23.code(),
        KeyCode::KEY_F24.code(),
        KeyCode::KEY_PLAYCD.code(),
        KeyCode::KEY_PAUSECD.code(),
        KeyCode::KEY_PROG3.code(),
        KeyCode::KEY_PROG4.code(),
        KeyCode::KEY_DASHBOARD.code(),
        KeyCode::KEY_SUSPEND.code(),
        KeyCode::KEY_CLOSE.code(),
        KeyCode::KEY_PLAY.code(),
        KeyCode::KEY_FASTFORWARD.code(),
        KeyCode::KEY_BASSBOOST.code(),
        KeyCode::KEY_PRINT.code(),
        KeyCode::KEY_HP.code(),
        KeyCode::KEY_CAMERA.code(),
        KeyCode::KEY_SOUND.code(),
        KeyCode::KEY_QUESTION.code(),
        KeyCode::KEY_EMAIL.code(),
        KeyCode::KEY_CHAT.code(),
        KeyCode::KEY_SEARCH.code(),
        KeyCode::KEY_CONNECT.code(),
        KeyCode::KEY_FINANCE.code(),
        KeyCode::KEY_SPORT.code(),
        KeyCode::KEY_SHOP.code(),
        KeyCode::KEY_ALTERASE.code(),
        KeyCode::KEY_CANCEL.code(),
        KeyCode::KEY_BRIGHTNESSDOWN.code(),
        KeyCode::KEY_BRIGHTNESSUP.code(),
        KeyCode::KEY_MEDIA.code(),
        KeyCode::KEY_SWITCHVIDEOMODE.code(),
        KeyCode::KEY_KBDILLUMTOGGLE.code(),
        KeyCode::KEY_KBDILLUMDOWN.code(),
        KeyCode::KEY_KBDILLUMUP.code(),
        KeyCode::KEY_SEND.code(),
        KeyCode::KEY_REPLY.code(),
        KeyCode::KEY_FORWARDMAIL.code(),
        KeyCode::KEY_SAVE.code(),
        KeyCode::KEY_DOCUMENTS.code(),
        KeyCode::KEY_BATTERY.code(),
        KeyCode::KEY_BLUETOOTH.code(),
        KeyCode::KEY_WLAN.code(),
        KeyCode::KEY_UWB.code(),
        KeyCode::KEY_UNKNOWN.code(),
        KeyCode::KEY_VIDEO_NEXT.code(),
        KeyCode::KEY_VIDEO_PREV.code(),
        KeyCode::KEY_BRIGHTNESS_CYCLE.code(),
        KeyCode::KEY_BRIGHTNESS_AUTO.code(),
        KeyCode::KEY_DISPLAY_OFF.code(),
        KeyCode::KEY_WWAN.code(),
        KeyCode::KEY_RFKILL.code(),
        KeyCode::KEY_MICMUTE.code(),
    ];

    pub fn get_all_keys_code() -> Vec<u16> {
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

        fn remove_event(&mut self, key_code: &u16) {
            self.container.remove(key_code);
        }

        fn last_timestamp(&self, key_code: &u16) -> Option<&SystemTime> {
            self.container.get(key_code)
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

    #[derive(Debug)]
    pub struct Device {
        vendor: u16,
        product: u16,
    }

    impl Device {
        pub fn new(vendor: u16, product: u16) -> Device {
            Device { vendor, product }
        }
    }

    pub fn should_skip(
        ev: &KeyEvent,
        key_holder: &mut KeyEventHolder,
        config_holder: &ConfigHolder,
    ) -> bool {
        if ev.value == 2 {
            return false;
        }
        if !config_holder.keys.contains(&ev.keycode) {
            return false;
        }
        match key_holder.last_timestamp(&ev.keycode) {
            Some(&timestamp) => {
                let should_skip =
                    ev.timestamp.duration_since(timestamp).unwrap() <= key_holder.minimum_delay;
                if ev.value == 1 {
                    if !should_skip {
                        key_holder.remove_event(&ev.keycode);
                        return false;
                    }
                } else {
                    return true;
                }
                should_skip
            }
            None => {
                if ev.value == 0 {
                    key_holder.insert_event(ev.keycode, ev.timestamp);
                }
                false
            }
        }
    }

    pub fn list_devices() -> Vec<DeviceEvDev> {
        evdev::enumerate()
            .map(|d| d.1)
            .collect::<Vec<DeviceEvDev>>()
    }

    pub fn receive_event(device: &mut DeviceEvDev) -> Vec<KeyEvent> {
        let result: Vec<InputEvent> = device.fetch_events().unwrap().collect();
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

    pub fn emit_key_event(event: KeyEvent, virtual_device: &mut VirtualDevice) {
        let key_event = *KeyEventEvDev::new(KeyCode(event.keycode), event.value);
        virtual_device.emit(&[key_event]).unwrap();
    }

    pub fn create_virtual_device() -> VirtualDevice {
        let mut keys = AttributeSet::<KeyCode>::new();
        for key in get_all_keys_code() {
            keys.insert(KeyCode::new(key));
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
    use crate::device::linux::debounce::get_all_keys_code;
    use config::{Config, File, FileFormat};
    use evdev::{AttributeSet, KeyCode};
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::io::Write;
    use std::path::PathBuf;

    const AVAILABLE_KEYS: [(&str, u16); 95] = [
        ("KEY_ESC", KeyCode::KEY_ESC.code()),
        ("KEY_1", KeyCode::KEY_1.code()),
        ("KEY_2", KeyCode::KEY_2.code()),
        ("KEY_3", KeyCode::KEY_3.code()),
        ("KEY_4", KeyCode::KEY_4.code()),
        ("KEY_5", KeyCode::KEY_5.code()),
        ("KEY_6", KeyCode::KEY_6.code()),
        ("KEY_7", KeyCode::KEY_7.code()),
        ("KEY_8", KeyCode::KEY_8.code()),
        ("KEY_9", KeyCode::KEY_9.code()),
        ("KEY_0", KeyCode::KEY_0.code()),
        ("KEY_MINUS", KeyCode::KEY_MINUS.code()),
        ("KEY_EQUAL", KeyCode::KEY_EQUAL.code()),
        ("KEY_BACKSPACE", KeyCode::KEY_BACKSPACE.code()),
        ("KEY_TAB", KeyCode::KEY_TAB.code()),
        ("KEY_Q", KeyCode::KEY_Q.code()),
        ("KEY_W", KeyCode::KEY_W.code()),
        ("KEY_E", KeyCode::KEY_E.code()),
        ("KEY_R", KeyCode::KEY_R.code()),
        ("KEY_T", KeyCode::KEY_T.code()),
        ("KEY_Y", KeyCode::KEY_Y.code()),
        ("KEY_U", KeyCode::KEY_U.code()),
        ("KEY_I", KeyCode::KEY_I.code()),
        ("KEY_O", KeyCode::KEY_O.code()),
        ("KEY_P", KeyCode::KEY_P.code()),
        ("KEY_LEFTBRACE", KeyCode::KEY_LEFTBRACE.code()),
        ("KEY_RIGHTBRACE", KeyCode::KEY_RIGHTBRACE.code()),
        ("KEY_ENTER", KeyCode::KEY_ENTER.code()),
        ("KEY_LEFTCTRL", KeyCode::KEY_LEFTCTRL.code()),
        ("KEY_A", KeyCode::KEY_A.code()),
        ("KEY_S", KeyCode::KEY_S.code()),
        ("KEY_D", KeyCode::KEY_D.code()),
        ("KEY_F", KeyCode::KEY_F.code()),
        ("KEY_G", KeyCode::KEY_G.code()),
        ("KEY_H", KeyCode::KEY_H.code()),
        ("KEY_J", KeyCode::KEY_J.code()),
        ("KEY_K", KeyCode::KEY_K.code()),
        ("KEY_L", KeyCode::KEY_L.code()),
        ("KEY_SEMICOLON", KeyCode::KEY_SEMICOLON.code()),
        ("KEY_APOSTROPHE", KeyCode::KEY_APOSTROPHE.code()),
        ("KEY_GRAVE", KeyCode::KEY_GRAVE.code()),
        ("KEY_LEFTSHIFT", KeyCode::KEY_LEFTSHIFT.code()),
        ("KEY_BACKSLASH", KeyCode::KEY_BACKSLASH.code()),
        ("KEY_Z", KeyCode::KEY_Z.code()),
        ("KEY_X", KeyCode::KEY_X.code()),
        ("KEY_C", KeyCode::KEY_C.code()),
        ("KEY_V", KeyCode::KEY_V.code()),
        ("KEY_B", KeyCode::KEY_B.code()),
        ("KEY_N", KeyCode::KEY_N.code()),
        ("KEY_M", KeyCode::KEY_M.code()),
        ("KEY_COMMA", KeyCode::KEY_COMMA.code()),
        ("KEY_DOT", KeyCode::KEY_DOT.code()),
        ("KEY_SLASH", KeyCode::KEY_SLASH.code()),
        ("KEY_RIGHTSHIFT", KeyCode::KEY_RIGHTSHIFT.code()),
        ("KEY_KPASTERISK", KeyCode::KEY_KPASTERISK.code()),
        ("KEY_LEFTALT", KeyCode::KEY_LEFTALT.code()),
        ("KEY_SPACE", KeyCode::KEY_SPACE.code()),
        ("KEY_CAPSLOCK", KeyCode::KEY_CAPSLOCK.code()),
        ("KEY_F1", KeyCode::KEY_F1.code()),
        ("KEY_F2", KeyCode::KEY_F2.code()),
        ("KEY_F3", KeyCode::KEY_F3.code()),
        ("KEY_F4", KeyCode::KEY_F4.code()),
        ("KEY_F5", KeyCode::KEY_F5.code()),
        ("KEY_F6", KeyCode::KEY_F6.code()),
        ("KEY_F7", KeyCode::KEY_F7.code()),
        ("KEY_F8", KeyCode::KEY_F8.code()),
        ("KEY_F9", KeyCode::KEY_F9.code()),
        ("KEY_F10", KeyCode::KEY_F10.code()),
        ("KEY_NUMLOCK", KeyCode::KEY_NUMLOCK.code()),
        ("KEY_SCROLLLOCK", KeyCode::KEY_SCROLLLOCK.code()),
        ("KEY_F11", KeyCode::KEY_F11.code()),
        ("KEY_F12", KeyCode::KEY_F12.code()),
        ("KEY_SYSRQ", KeyCode::KEY_SYSRQ.code()),
        ("KEY_RIGHTALT", KeyCode::KEY_RIGHTALT.code()),
        ("KEY_HOME", KeyCode::KEY_HOME.code()),
        ("KEY_UP", KeyCode::KEY_UP.code()),
        ("KEY_PAGEUP", KeyCode::KEY_PAGEUP.code()),
        ("KEY_LEFT", KeyCode::KEY_LEFT.code()),
        ("KEY_RIGHT", KeyCode::KEY_RIGHT.code()),
        ("KEY_END", KeyCode::KEY_END.code()),
        ("KEY_DOWN", KeyCode::KEY_DOWN.code()),
        ("KEY_PAGEDOWN", KeyCode::KEY_PAGEDOWN.code()),
        ("KEY_INSERT", KeyCode::KEY_INSERT.code()),
        ("KEY_DELETE", KeyCode::KEY_DELETE.code()),
        ("KEY_PAUSE", KeyCode::KEY_PAUSE.code()),
        ("KEY_NUMERIC_0", KeyCode::KEY_NUMERIC_0.code()),
        ("KEY_NUMERIC_1", KeyCode::KEY_NUMERIC_1.code()),
        ("KEY_NUMERIC_2", KeyCode::KEY_NUMERIC_2.code()),
        ("KEY_NUMERIC_3", KeyCode::KEY_NUMERIC_3.code()),
        ("KEY_NUMERIC_4", KeyCode::KEY_NUMERIC_4.code()),
        ("KEY_NUMERIC_5", KeyCode::KEY_NUMERIC_5.code()),
        ("KEY_NUMERIC_6", KeyCode::KEY_NUMERIC_6.code()),
        ("KEY_NUMERIC_7", KeyCode::KEY_NUMERIC_7.code()),
        ("KEY_NUMERIC_8", KeyCode::KEY_NUMERIC_8.code()),
        ("KEY_NUMERIC_9", KeyCode::KEY_NUMERIC_9.code()),
    ];

    pub struct ConfigHolder {
        pub keys: Vec<u16>,
        pub device_id: u32,
        pub device_name: String,
        pub delay_ms: u64,
    }

    impl Display for ConfigHolder {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let keys = AVAILABLE_KEYS
                .into_iter()
                .filter(|(_, v)| self.keys.contains(v))
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

    fn get_keys_code(keys: &str) -> Vec<u16> {
        let available_keys_map = AVAILABLE_KEYS.into_iter().collect::<HashMap<&str, u16>>();
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
