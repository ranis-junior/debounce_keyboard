#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use crate::device::linux::{command_line, config, debounce};

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use crate::device::windows::{command_line, config, debounce};
