Rust PulsecAudio API
====================

A Rust api wrapper around `libpulse_binding` to make pulseaudio application development easier.
This is a wrapper around the introspector, and thus this library is only capable of modifying PulseAudio data (changing volume, routing applications and muting right now).

### Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
rust-pulsectl = "0.2"
```

Then, connect to PulseAudio by creating a `SinkController` for audio playback devices and apps or a `SourceController` for audio recording devices and apps.

```rust
// See examples/change_device_vol.rs for a more complete example
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
}
```

