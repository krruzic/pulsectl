use crate::Handler;
use pulse::callbacks::ListResult;
use pulse::context::introspect::SourceInfo;
use pulse::volume::Volume;
use pulse::volume::{ChannelVolumes, VolumeLinear};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct SimpleSourceDevice {
    pub index: u32,
    pub volume: ChannelVolumes,
    pub name: String,
}

pub struct SourceController {
    pub handler: Handler,
}

impl SourceController {
    pub fn create() -> Self {
        let handler = Handler::connect("SourceController").unwrap();
        SourceController { handler: handler }
    }
}

impl super::DeviceControl<'_, SimpleSourceDevice> for SourceController {
    fn list_devices(&mut self) -> Rc<RefCell<Vec<SimpleSourceDevice>>> {
        let inputs: Rc<RefCell<Vec<SimpleSourceDevice>>> = Rc::new(RefCell::new(Vec::new()));
        let input_refs = inputs.clone();
        let op = self.handler.introspect.get_source_info_list(
            move |source_list: ListResult<&SourceInfo>| {
                if let ListResult::Item(item) = source_list {
                    if let Some(name) = &item.name {
                        input_refs.borrow_mut().push(SimpleSourceDevice {
                            index: item.index,
                            name: name.to_string().clone(),
                            volume: item.volume,
                        })
                    }
                }
            },
        );
        self.handler.wait_for_operation(op).expect("error");
        inputs
    }

    fn get_device_by_index(&mut self, index: u32) -> Rc<RefCell<SimpleSourceDevice>> {
        let input: Rc<RefCell<SimpleSourceDevice>> = Rc::new(RefCell::new(Default::default()));
        let input_ref = input.clone();
        let op = self.handler.introspect.get_source_info_by_index(
            index,
            move |source_list: ListResult<&SourceInfo>| {
                if let ListResult::Item(item) = source_list {
                    if let Some(name) = &item.name {
                        input_ref.borrow_mut().index = item.index;
                        input_ref.borrow_mut().name = name.to_string().clone();
                        input_ref.borrow_mut().volume = item.volume;
                    }
                }
            },
        );
        self.handler.wait_for_operation(op).expect("error");
        input
    }

    fn get_device_by_name(&mut self, name: &str) -> Rc<RefCell<SimpleSourceDevice>> {
        let input: Rc<RefCell<SimpleSourceDevice>> = Rc::new(RefCell::new(Default::default()));
        let input_ref = input.clone();
        let op = self.handler.introspect.get_source_info_by_name(
            name,
            move |source_list: ListResult<&SourceInfo>| {
                if let ListResult::Item(item) = source_list {
                    if let Some(name) = &item.name {
                        input_ref.borrow_mut().index = item.index;
                        input_ref.borrow_mut().name = name.to_string().clone();
                        input_ref.borrow_mut().volume = item.volume;
                    }
                }
            },
        );
        self.handler.wait_for_operation(op).expect("error");
        input
    }
    fn set_device_volume_by_name(&mut self, name: &str, volume: &ChannelVolumes) {
        let op = self
            .handler
            .introspect
            .set_source_volume_by_name(name, volume, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn get_default_device(&mut self) -> Rc<RefCell<SimpleSourceDevice>> {
        let input: Rc<RefCell<String>> = Rc::new(RefCell::new(Default::default()));
        let input_ref = input.clone();
        let op = self.handler.introspect.get_server_info(move |res| {
            if let Some(default_source_name) = &res.default_source_name {
                input_ref
                    .borrow_mut()
                    .push_str(&default_source_name.clone().to_string());
                //                input_ref
                //                    .borrow_mut()
                //                    .clone_from(&default_source_name.clone().to_string())
            }
        });
        self.handler.wait_for_operation(op).expect("error");
        self.get_device_by_name(&input.clone().borrow_mut())
    }
    fn set_device_volume_by_index(&mut self, index: u32, volume: &ChannelVolumes) {
        let op = self
            .handler
            .introspect
            .set_source_volume_by_index(index, volume, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn set_default_device(&mut self, name: &str) -> Rc<RefCell<bool>> {
        let input: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        let input_ref = input.clone();
        let op = self
            .handler
            .context
            .borrow_mut()
            .set_default_source(name, move |res| input_ref.borrow_mut().clone_from(&res));
        self.handler.wait_for_operation(op).expect("error");
        input
    }

    fn increase_device_volume_by_percent(&mut self, index: u32, delta: f64) {
        let dev_ref = self.get_device_by_index(index).clone();
        let mut device = dev_ref.borrow_mut();
        let new_vol = Volume::from(Volume(delta as u32));
        let volumes = device.volume.increase(new_vol).unwrap();
        let op = self
            .handler
            .introspect
            .set_source_volume_by_index(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn decrease_device_volume_by_percent(&mut self, index: u32, delta: f64) {
        let dev_ref = self.get_device_by_index(index).clone();
        let mut device = dev_ref.borrow_mut();
        let new_vol = Volume::from(VolumeLinear(delta));
        let volumes = device.volume.decrease(new_vol).unwrap();
        let op = self
            .handler
            .introspect
            .set_source_volume_by_index(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }
}
