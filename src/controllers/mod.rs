
use pulse::volume::ChannelVolumes;



pub mod sink_controller;
pub mod source_controller;
pub mod types;

// Source = microphone etc. something that takes in audio
// Source Output = application consuming that audio
//
// Sink = headphones etc. something that plays out audio
// Sink Input = application producing that audio
pub trait DeviceControl<T> {
    fn get_default_device(&mut self) -> Result<T, ()>;
    fn set_default_device(&mut self, name: &str) -> Result<bool, ()>;

    fn list_devices(&mut self) -> Result<Vec<T>, ()>;
    fn get_device_by_index(&mut self, index: u32) -> Result<T, ()>;
    fn get_device_by_name(&mut self, name: &str) -> Result<T, ()>;
    fn set_device_volume_by_index(&mut self, index: u32, volume: &ChannelVolumes);
    fn set_device_volume_by_name(&mut self, name: &str, volume: &ChannelVolumes);
    fn increase_device_volume_by_percent(&mut self, index: u32, delta: f64);
    fn decrease_device_volume_by_percent(&mut self, index: u32, delta: f64);
}

pub trait AppControl<T> {
    fn list_applications(&mut self) -> Result<Vec<T>, ()>;

    fn get_app_by_index(&mut self, index: u32) -> Result<T, ()>;
    fn increase_app_volume_by_percent(&mut self, index: u32, delta: f64);
    fn decrease_app_volume_by_percent(&mut self, index: u32, delta: f64);

    fn move_app_by_index(&mut self, stream_index: u32, device_index: u32) -> Result<bool, ()>;
    fn move_app_by_name(&mut self, stream_index: u32, device_name: &str) -> Result<bool, ()>;
    fn set_app_mute(&mut self, index: u32, mute: bool) -> Result<bool, ()>;
}
fn volume_from_percent(volume: f64) -> f64 {
    ((volume * 100.0) * (f64::from(pulse::volume::VOLUME_NORM.0) / 100.0))
}
