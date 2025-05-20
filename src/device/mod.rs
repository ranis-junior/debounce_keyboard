#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::{command_line, config, debounce};

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::{
    command_line, config,
    debounce::{run_message_loop, setup_windows_ll_keyboard_hook, KeyEventHolder},
};

pub use clap::Parser;
pub use std::process::exit;
