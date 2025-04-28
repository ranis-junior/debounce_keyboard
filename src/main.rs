extern crate core;

use crate::device::command_line::{Cli, Commands};
use crate::device::config::{ConfigHolder, load_config, save_config_to_path};
use crate::device::debounce::{
    KeyEventHolder, combine_u16_to_u32, create_virtual_device, emit_key_event, list_devices,
    receive_event, should_skip, split_u32_to_u16,
};
use clap::Parser;
use evdev::{Device, EventSummary};
use std::process::exit;

mod device;

#[cfg(target_os = "linux")]
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
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Invalid argument: should be a number")),
        },
        None => 0,
    };

    let mut config: ConfigHolder = load_config(&config_path);

    let mut device: Device;
    if device_number == 0 && config.device_id == 0 {
        eprintln!(
            "No device provided: use the select option or provide 'device_id' in config file."
        );
        exit(1);
    } else if device_number == 0 && config.device_id > 0 {
        let (vendor, product): (u16, u16) = split_u32_to_u16(config.device_id);
        device = devices
            .into_iter()
            .find(|d| {
                d.input_id().vendor() == vendor
                    && d.input_id().product() == product
                    && d.name().unwrap() == config.device_name
            })
            .expect("No devices found!");
    } else {
        device = devices
            .into_iter()
            .nth(device_number as usize)
            .expect("Invalid device number!");
        let (vendor, product): (u16, u16) =
            (device.input_id().vendor(), device.input_id().product());
        config.device_id = combine_u16_to_u32(vendor, product);
        config.device_name = device.name().expect("Unknow device name!").to_owned();
        save_config_to_path(&config_path.clone(), &config);
    }

    let mut key_event_holder = KeyEventHolder::new(config.delay_ms);
    let mut virtual_device = create_virtual_device();

    device.grab().unwrap();
    println!(
        "Watching {} for key events",
        device.name().unwrap_or("Unknown device")
    );
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
