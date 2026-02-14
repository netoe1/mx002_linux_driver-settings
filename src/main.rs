mod virtual_device;
mod physical_device;
mod config;
use signal_hook::consts::signal::*;
use signal_hook::flag::register;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::u16;
use std::io;   
use physical_device::PhysicalDevice;
use virtual_device::{DeviceDispatcher, RawDataReader};

// Interface to change variables in config file.
// use crate::{ config::check_if_has_root_access, config::set_pen_strength, config::set_pen_threshold};

const VID: u16 = 0x08f2;
const PID: u16 = 0x6811;


fn main() {
    // netoe1-mod: Verifying if program is running as Sudo!

    config::check_if_has_root_access();

    let mut physical_device = PhysicalDevice::new(VID, PID);
    physical_device.init().set_full_mode();

    let mut data_reader = RawDataReader::new();
    let mut device_dispatcher = DeviceDispatcher::new();

    // netoe1-mod: Adding config interface
    // These values control pen proximity and pressure sensitivity

    let mut get_threshold_proximity = String::new();
    let mut get_strength_scaling = String::new();

    // --- Threshold ---
    println!("Set threshold proximity [0-1700], recommended 600:");
    io::stdin()
        .read_line(&mut get_threshold_proximity)
        .expect("mx002-driver-err: Failed to read threshold");

    let threshold: i32 = get_threshold_proximity
        .trim()
        .parse()
        .expect("mx002-driver-err: Threshold must be an integer");

    // Optional safety clamp
    let threshold = threshold.clamp(0, 1700);

    // --- Strength ---
    println!("Set strength scaling [1-10], recommended 2:");
    io::stdin()
        .read_line(&mut get_strength_scaling)
        .expect("mx002-driver-err: Failed to read strength");

    let strength: i32 = get_strength_scaling
        .trim()
        .parse()
        .expect("mx002-driver-err: Strength must be an integer");

    let strength = strength.clamp(1, 10);

    println!("mx002-driver-log: Values typed below!");
    println!("mx002-driver-log: Configured sens.");
    println!("mx002-driver-log: Threshold proximity = {}", threshold);
    println!("mx002-driver-log: Strength scaling = {}", strength);

    // Setting values.
    config::set_pen_strength(strength);
    config::set_pen_threshold(threshold);

    println!("mx002-driver-log: The driver is running!");

    main_loop({
        || {
            if physical_device
                .read_device_responses(&mut data_reader.data)
                .is_ok()
            {
                device_dispatcher.dispatch(&data_reader);
                if device_dispatcher.syn().is_err() {
                    println!("mx002-driver-err:Error emitting SYN.");
                }
            }
        }
    });
}

fn main_loop(mut f: impl FnMut()) {
    let signals: Vec<i32> = vec![SIGINT, SIGTERM, SIGQUIT];
    let flag = Arc::new(AtomicBool::new(false));

    for signal in signals {
        register(signal, Arc::clone(&flag)).expect("Error registering interrupt signals.");
    }

    while !flag.load(Ordering::Relaxed) {
        f();
    }
    println!("mx002-driver-info: The driver is exiting.");
}