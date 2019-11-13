extern crate pulsectl;

use std::io;

use pulsectl::controllers::sink_controller::{SimpleSinkDevice, SimpleSinkStream, SinkController};
use pulsectl::controllers::{AppControl, DeviceControl};
fn main() {
    let mut handler = SinkController::create(); // create handler that calls functions on playback devices and apps
    let dev_ref = handler.list_devices().clone();
    let dev_borrow = dev_ref.borrow();
    let playback_dev_unwrapped = dev_borrow.iter().map(|x| x).collect::<Vec<_>>();

    println!("Playback Devices");
    for dev in playback_dev_unwrapped.clone() {
        println!(
            "[{}] {}, Volume: {}",
            dev.index,
            dev.description.clone(),
            dev.volume.print()
        );
    }
    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("error: unable to read user input");
    for dev in playback_dev_unwrapped {
        match selection.trim() == dev.index.to_string() {
            true => {
                println!("hey");
                handler.increase_device_volume_by_percent(dev.index, 0.05);
            }
            _ => {}
        }
    }
}
