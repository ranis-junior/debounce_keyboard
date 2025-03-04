extern crate core;

use crate::device::command_line::{Cli, Commands};
use crate::device::config::{ConfigHolder, load_config, save_config_to_path};
use crate::device::debounce::{
    KeyEventHolder, create_virtual_device, emit_key_event, list_devices, receive_event, should_skip,
};
use clap::Parser;
use evdev::EventSummary;
use std::process::exit;

mod device;

fn main() {
    let args = Cli::parse();

    let config_path = args.config_path.unwrap_or_else(|| "config.ini".into());

    let devices = list_devices();
    let device_number = match args.command {
        Some(command) => match command {
            Commands::ListDevices => {
                for (i, device) in devices.iter().enumerate() {
                    println!("{:?}: {}", i, device.name().unwrap_or("Unknown device"));
                }
                exit(0);
            }
            Commands::Select { device } => device
                .trim()
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Invalid argument: should be a number")),
        },
        None => 0,
    };

    let mut config: ConfigHolder;
    if !config_path.exists() {
        if device_number == 0 {
            eprintln!(
                "No device provided: use the select option or provide 'device_id' in config file."
            );
            exit(1);
        }
        std::fs::File::create(&config_path).expect("Failed to create config file");
        config = load_config(&config_path);
        config.device_id = device_number.to_string();
        save_config_to_path(&config_path.clone(), &config);
    } else {
        config = load_config(&config_path);
        let device_number_str = device_number.to_string();
        if device_number != 0 && config.device_id != device_number_str {
            config.device_id = device_number_str;
            save_config_to_path(&config_path, &config);
        }
    }

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
