use std::cell::RefCell;
use std::rc::Rc;

use pulse::callbacks::ListResult;
use pulse::context::introspect::{SinkInfo, SinkInputInfo};
use pulse::volume::ChannelVolumes;
use pulse::volume::Volume;

use crate::Handler;

pub struct SinkController {
    pub handler: Handler,
    //    pub sink_type: PhantomData<T>,
}

impl SinkController {
    pub fn create() -> Self {
        let handler = Handler::connect("SinkController").unwrap();
        SinkController { handler }
    }
}

#[derive(Default, Clone)]
pub struct SimpleSinkDevice {
    pub index: u32,
    pub volume: ChannelVolumes,
    pub name: String,
    pub description: String,
}

impl super::DeviceControl<'_, SimpleSinkDevice> for SinkController {
    fn list_devices(&mut self) -> Rc<RefCell<Vec<SimpleSinkDevice>>> {
        let inputs: Rc<RefCell<Vec<SimpleSinkDevice>>> = Rc::new(RefCell::new(Vec::new()));
        let input_refs = inputs.clone();
        let op =
            self.handler
                .introspect
                .get_sink_info_list(move |sink_list: ListResult<&SinkInfo>| {
                    if let ListResult::Item(item) = sink_list {
                        if let Some(name) = &item.name {
                            if let Some(description) = &item.description {
                                input_refs.borrow_mut().push(SimpleSinkDevice {
                                    index: item.index,
                                    name: name.to_string().clone(),
                                    description: description.to_string().clone(),
                                    volume: item.volume,
                                })
                            }
                        }
                    }
                });
        self.handler.wait_for_operation(op).expect("error");
        inputs
    }
    fn get_device_by_index(&mut self, index: u32) -> Rc<RefCell<SimpleSinkDevice>> {
        let input: Rc<RefCell<SimpleSinkDevice>> = Rc::new(RefCell::new(Default::default()));
        let input_ref = input.clone();
        let op = self.handler.introspect.get_sink_info_by_index(
            index,
            move |sink_list: ListResult<&SinkInfo>| {
                if let ListResult::Item(item) = sink_list {
                    if let Some(description) = &item.description {
                        input_ref.borrow_mut().index = item.index;
                        input_ref.borrow_mut().description = description.to_string().clone();
                        input_ref.borrow_mut().volume = item.volume;
                    }
                }
            },
        );
        self.handler.wait_for_operation(op).expect("error");
        input
    }
    fn get_device_by_name(&mut self, name: &str) -> Rc<RefCell<SimpleSinkDevice>> {
        let input: Rc<RefCell<SimpleSinkDevice>> = Rc::new(RefCell::new(Default::default()));
        let input_ref = input.clone();
        let op = self.handler.introspect.get_sink_info_by_name(
            name,
            move |sink_list: ListResult<&SinkInfo>| {
                if let ListResult::Item(item) = sink_list {
                    if let Some(description) = &item.description {
                        input_ref.borrow_mut().index = item.index;
                        input_ref.borrow_mut().description = description.to_string().clone();
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
            .set_sink_volume_by_name(name, volume, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn get_default_device(&mut self) -> Rc<RefCell<SimpleSinkDevice>> {
        let input: Rc<RefCell<String>> = Rc::new(RefCell::new(Default::default()));
        let input_ref = input.clone();
        let op = self.handler.introspect.get_server_info(move |res| {
            if let Some(default_sink_name) = &res.default_sink_name {
                input_ref
                    .borrow_mut()
                    .push_str(&default_sink_name.clone().to_string());
            }
        });
        self.handler.wait_for_operation(op).expect("error");
        self.get_device_by_name(&input.clone().borrow_mut())
    }

    fn set_device_volume_by_index(&mut self, index: u32, volume: &ChannelVolumes) {
        let op = self
            .handler
            .introspect
            .set_sink_volume_by_index(index, volume, None);
        self.handler.wait_for_operation(op).expect("error");
    }

    fn increase_device_volume_by_percent(&mut self, index: u32, delta: f64) {
        let dev_ref = self.get_device_by_index(index).clone();
        let mut device = dev_ref.borrow_mut();
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = device.volume.increase(new_vol).unwrap();
        let op = self
            .handler
            .introspect
            .set_sink_volume_by_index(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }

    fn decrease_device_volume_by_percent(&mut self, index: u32, delta: f64) {
        let dev_ref = self.get_device_by_index(index).clone();
        let mut device = dev_ref.borrow_mut();
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = device.volume.decrease(new_vol).unwrap();
        let op = self
            .handler
            .introspect
            .set_sink_volume_by_index(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn set_default_device(&mut self, name: &str) -> Rc<RefCell<bool>> {
        let input: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        let input_ref = input.clone();
        let op = self
            .handler
            .context
            .borrow_mut()
            .set_default_sink(name, move |res| input_ref.borrow_mut().clone_from(&res));
        self.handler.wait_for_operation(op).expect("error");
        input
    }
}

#[derive(Default, Clone)]
pub struct SimpleSinkStream {
    pub index: u32,
    pub vol: ChannelVolumes,
    pub name: String,
    pub description: String,
    pub device_index: u32,
    pub corked: bool,
}

impl super::AppControl<'_, SimpleSinkStream> for SinkController {
    fn list_applications(&mut self) -> Rc<RefCell<Vec<SimpleSinkStream>>> {
        let inputs: Rc<RefCell<Vec<SimpleSinkStream>>> = Rc::new(RefCell::new(Vec::new()));
        let input_refs = inputs.clone();
        let op = self.handler.introspect.get_sink_input_info_list(
            move |source_list: ListResult<&SinkInputInfo>| {
                if let ListResult::Item(item) = source_list {
                    if let Some(name) = &item.name {
                        input_refs.borrow_mut().push(SimpleSinkStream {
                            index: item.index,
                            name: name.to_string().clone(),
                            description: item
                                .proplist
                                .get_str("application.process.binary")
                                .unwrap(),
                            vol: item.volume,
                            corked: item.corked,
                            device_index: item.sink,
                        })
                    }
                }
            },
        );
        self.handler.wait_for_operation(op).expect("error");
        inputs
    }
    fn get_app_by_index(&mut self, index: u32) -> Rc<RefCell<SimpleSinkStream>> {
        let input: Rc<RefCell<SimpleSinkStream>> = Rc::new(RefCell::new(Default::default()));
        let input_ref = input.clone();
        let op = self.handler.introspect.get_sink_input_info(
            index,
            move |sink_input_list: ListResult<&SinkInputInfo>| {
                if let ListResult::Item(item) = sink_input_list {
                    if let Some(name) = &item.name {
                        input_ref.borrow_mut().index = item.index;
                        input_ref.borrow_mut().name = name.to_string().clone();
                        input_ref.borrow_mut().vol = item.volume;
                        input_ref.borrow_mut().corked = item.corked;
                    }
                }
            },
        );
        self.handler.wait_for_operation(op).expect("error");
        input
    }
    fn move_app_by_index(&mut self, stream_index: u32, device_index: u32) -> Rc<RefCell<bool>> {
        let input: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        let input_ref = input.clone();
        let op = self.handler.introspect.move_sink_input_by_index(
            stream_index,
            device_index,
            Some(Box::new(move |res| input_ref.borrow_mut().clone_from(&res))),
        );
        self.handler.wait_for_operation(op).expect("error");
        input
    }
    fn increase_app_volume_by_percent(&mut self, index: u32, delta: f64) {
        let app_ref = self.get_app_by_index(index).clone();
        let mut app = app_ref.borrow_mut();
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = app.vol.increase(new_vol).unwrap();
        let op = self
            .handler
            .introspect
            .set_sink_input_volume(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }

    fn decrease_app_volume_by_percent(&mut self, index: u32, delta: f64) {
        let app_ref = self.get_app_by_index(index).clone();
        let mut app = app_ref.borrow_mut();
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = app.vol.decrease(new_vol).unwrap();
        let op = self
            .handler
            .introspect
            .set_sink_input_volume(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }
}
