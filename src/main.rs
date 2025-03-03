extern crate core;

use crate::device::command_line::{Cli, Commands};
use crate::device::config::{ConfigHolder, load_config};
use crate::device::debounce::{
    KeyEventHolder, create_virtual_device, emit_key_event, list_devices, receive_event, should_skip,
};
use clap::Parser;
use config::{Config, File, FileFormat};
use evdev::EventSummary;
use std::io::Write;
use std::process::exit;

mod device;

fn main() {
    let args = Cli::parse();

    if args.list_devices {
        let devices = list_devices();
        for (i, device) in devices.iter().enumerate() {
            println!("{:?}: {}", i, device.name().unwrap_or("Unknown device"));
        }
        exit(0);
    }

    let device_number = match args.command {
        Some(Commands::Select { device }) => device
            .trim()
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Invalid argument: should be a number",)),
        None => exit(0),
    };

    let config_path = args.config_path.unwrap_or_else(|| "config.ini".into());

    let mut config: ConfigHolder;
    if !config_path.exists() {
        let mut config_file =
            std::fs::File::create(&config_path).expect("Failed to create config.ini file");
        config = load_config(config_path);
        config.device_id = device_number.to_string();
        config_file
            .write_all(&config.to_string().as_bytes())
            .expect("Failed to write config.ini file");
    } else {
        config = load_config(config_path);
    }

    let mut devices = list_devices();
    let mut key_event_holder = KeyEventHolder::new(config.delay_ms);
    let mut device = devices
        .into_iter()
        .nth(device_number)
        .expect("Invalid device number!");
    let mut virtual_device = create_virtual_device();

    device.grab().unwrap();
    loop {
        let fetched_events = receive_event(&mut device);
        for event in fetched_events {
            if let EventSummary::Key(event, _, _) = event.destructure() {
                if !should_skip(&event, &mut key_event_holder, &config) {
                    #[cfg(debug_assertions)]
                    println!("{:?}", event);
                    emit_key_event(event.code().code(), event.value(), &mut virtual_device)
                }
            }
        }
    }
}
