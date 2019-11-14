extern crate pulsectl;

use std::io;

use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;

fn main() {
    let mut handler = SinkController::create(); // create handler that calls functions on playback devices and apps
    let devices = handler
        .list_devices()
        .expect("Could not get list of playback devices");

    println!("Playback Devices");
    for dev in devices.clone() {
        println!(
            "[{}] {}, Volume: {}",
            dev.index,
            dev.description.as_ref().unwrap(),
            dev.volume.print()
        );
    }
    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("error: unable to read user input");
    for dev in devices.clone() {
        if let true = selection.trim() == dev.index.to_string() {
            println!("hey");
            handler.increase_device_volume_by_percent(100, 0.05);
        }
    }
}
