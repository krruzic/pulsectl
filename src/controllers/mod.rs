use pulse::volume::ChannelVolumes;
use std::cell::RefCell;
use std::rc::Rc;
pub mod sink_controller;
pub mod source_controller;

// Source = microphone etc. something that takes in audio
// Source_Output = application consuming that audio
//
// Sink = headphones etc. something that plays out audio
// Sink Input = application producing that audio

#[derive(Default, Clone)]
pub struct SimpleServerInfo {
    pub default_sink: String,
    pub default_source: String,
    pub cookie: u32,
}

fn volume_from_percent(volume: f64) -> f64 {
    ((volume * 100.0) * (f64::from(pulse::volume::VOLUME_NORM.0) / 100.0))
}

pub trait DeviceControl<'a, T> {
    fn list_devices(&'a mut self) -> Rc<RefCell<Vec<T>>>;
    fn get_device_by_index(&mut self, index: u32) -> Rc<RefCell<T>>;
    fn get_device_by_name(&mut self, name: &str) -> Rc<RefCell<T>>;
    fn set_device_volume_by_name(&mut self, name: &str, volume: &ChannelVolumes);
    fn get_default_device(&mut self) -> Rc<RefCell<T>>;
    fn set_device_volume_by_index(&mut self, index: u32, volume: &ChannelVolumes);
    fn increase_device_volume_by_percent(&mut self, index: u32, delta: f64);
    fn decrease_device_volume_by_percent(&mut self, index: u32, delta: f64);
    fn set_default_device(&mut self, name: &str) -> Rc<RefCell<bool>>;
}

pub trait AppControl<'a, T> {
    fn list_applications(&'a mut self) -> Rc<RefCell<Vec<T>>>;
    fn get_app_by_index(&mut self, index: u32) -> Rc<RefCell<T>>;
    fn increase_app_volume_by_percent(&mut self, index: u32, delta: f64);
    fn decrease_app_volume_by_percent(&mut self, index: u32, delta: f64);
    fn move_app_by_index(&mut self, stream_index: u32, device_index: u32) -> Rc<RefCell<bool>>;
}
