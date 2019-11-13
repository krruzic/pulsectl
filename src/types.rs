use crate::controllers::sink_controller::{SimpleSinkDevice, SimpleSinkStream};
use crate::controllers::source_controller::SimpleSourceDevice;
impl std::fmt::Debug for SimpleSinkDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sink Device {{ index: {}, name: {} }}",
            self.index, self.name
        )
    }
}

impl std::fmt::Debug for SimpleSinkStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sink App {{ index: {}, name: {}, corked: {} }}",
            self.index, self.name, self.corked
        )
    }
}
impl std::fmt::Debug for SimpleSourceDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Source Device {{ index: {}, name: {} }}",
            self.index, self.name
        )
    }
}
