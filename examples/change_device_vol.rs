extern crate pulsectl;

use std::io;

use pulsectl::controllers::sink_controller::SinkController;
use pulsectl::controllers::DeviceControl;
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
        match selection.trim() == dev.index.to_string() {
            true => {
                println!("hey");
                handler.increase_device_volume_by_percent(dev.index, 0.05);
            }
            _ => {}
        }
    }
}
