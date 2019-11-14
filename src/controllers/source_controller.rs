use std::cell::RefCell;
use std::clone::Clone;

use std::rc::Rc;

use pulse::callbacks::ListResult;
use pulse::context::introspect;


use pulse::volume::{ChannelVolumes, Volume};


use crate::controllers::types::{ApplicationInfo, ServerInfo};
use crate::Handler;

use super::types;


pub struct SourceController {
    pub handler: Handler,
}

impl SourceController {
    pub fn create() -> Self {
        let handler = Handler::connect("SourceController").unwrap();
        SourceController { handler }
    }

    pub fn get_server_info(&mut self) -> Result<ServerInfo, ()> {
        let server = Rc::new(RefCell::new(Some(None)));
        let server_ref = server.clone();

        let op = self.handler.introspect.get_server_info(move |res| {
            server_ref
                .borrow_mut()
                .as_mut()
                .unwrap()
                .replace(res.into());
        });
        self.handler.wait_for_operation(op)?;
        let mut result = server.borrow_mut();
        result.take().unwrap().ok_or(())
    }
}

impl super::DeviceControl<types::DeviceInfo> for SourceController {
    fn get_default_device(&mut self) -> Result<types::DeviceInfo, ()> {
        let server_info = self.get_server_info();
        match server_info {
            Ok(info) => self.get_device_by_name(info.default_sink_name.unwrap().as_ref()),
            Err(_) => Err(()),
        }
    }
    fn set_default_device(&mut self, name: &str) -> Result<bool, ()> {
        let success = Rc::new(RefCell::new(false));
        let success_ref = success.clone();

        let op = self
            .handler
            .context
            .borrow_mut()
            .set_default_sink(name, move |res| success_ref.borrow_mut().clone_from(&res));
        self.handler.wait_for_operation(op)?;
        let result = success.borrow_mut().clone();
        Ok(result)
    }

    fn list_devices(&mut self) -> Result<Vec<types::DeviceInfo>, ()> {
        let list = Rc::new(RefCell::new(Some(Vec::new())));
        let list_ref = list.clone();

        let op = self.handler.introspect.get_source_info_list(
            move |sink_list: ListResult<&introspect::SourceInfo>| {
                if let ListResult::Item(item) = sink_list {
                    list_ref.borrow_mut().as_mut().unwrap().push(item.into());
                }
            },
        );
        self.handler.wait_for_operation(op)?;
        let mut result = list.borrow_mut();
        result.take().ok_or(())
    }
    fn get_device_by_index(&mut self, index: u32) -> Result<types::DeviceInfo, ()> {
        let device = Rc::new(RefCell::new(Some(None)));
        let dev_ref = device.clone();
        let op = self.handler.introspect.get_source_info_by_index(
            index,
            move |sink_list: ListResult<&introspect::SourceInfo>| {
                if let ListResult::Item(item) = sink_list {
                    dev_ref.borrow_mut().as_mut().unwrap().replace(item.into());
                }
            },
        );
        self.handler.wait_for_operation(op)?;
        let mut result = device.borrow_mut();
        result.take().unwrap().ok_or(())
    }
    fn get_device_by_name(&mut self, name: &str) -> Result<types::DeviceInfo, ()> {
        let device = Rc::new(RefCell::new(Some(None)));
        let dev_ref = device.clone();
        let op = self.handler.introspect.get_source_info_by_name(
            name,
            move |sink_list: ListResult<&introspect::SourceInfo>| {
                if let ListResult::Item(item) = sink_list {
                    dev_ref.borrow_mut().as_mut().unwrap().replace(item.into());
                }
            },
        );
        self.handler.wait_for_operation(op)?;
        let mut result = device.borrow_mut();
        result.take().unwrap().ok_or(())
    }

    fn set_device_volume_by_index(&mut self, index: u32, volume: &ChannelVolumes) {
        let op = self
            .handler
            .introspect
            .set_source_volume_by_index(index, volume, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn set_device_volume_by_name(&mut self, name: &str, volume: &ChannelVolumes) {
        let op = self
            .handler
            .introspect
            .set_source_volume_by_name(name, volume, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn increase_device_volume_by_percent(&mut self, index: u32, delta: f64) {
        let mut dev_ref = self
            .get_device_by_index(index)
            .expect("Could not find device specified");
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = dev_ref
            .volume
            .increase(new_vol)
            .expect("Volume couldn't be set");
        let op = self
            .handler
            .introspect
            .set_source_volume_by_index(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }
    fn decrease_device_volume_by_percent(&mut self, index: u32, delta: f64) {
        let mut dev_ref = self
            .get_device_by_index(index)
            .expect("Could not find device specified");
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = dev_ref.volume.decrease(new_vol).unwrap();
        let op = self
            .handler
            .introspect
            .set_source_volume_by_index(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }
}

impl super::AppControl<types::ApplicationInfo> for SourceController {
    fn list_applications(&mut self) -> Result<Vec<ApplicationInfo>, ()> {
        let list = Rc::new(RefCell::new(Some(Vec::new())));
        let list_ref = list.clone();

        let op = self.handler.introspect.get_source_output_info_list(
            move |sink_list: ListResult<&introspect::SourceOutputInfo>| {
                if let ListResult::Item(item) = sink_list {
                    list_ref.borrow_mut().as_mut().unwrap().push(item.into());
                }
            },
        );
        self.handler.wait_for_operation(op)?;
        let mut result = list.borrow_mut();
        result.take().ok_or(())
    }

    fn get_app_by_index(&mut self, index: u32) -> Result<ApplicationInfo, ()> {
        let app = Rc::new(RefCell::new(Some(None)));
        let app_ref = app.clone();
        let op = self.handler.introspect.get_source_output_info(
            index,
            move |sink_list: ListResult<&introspect::SourceOutputInfo>| {
                if let ListResult::Item(item) = sink_list {
                    app_ref.borrow_mut().as_mut().unwrap().replace(item.into());
                }
            },
        );
        self.handler.wait_for_operation(op)?;
        let mut result = app.borrow_mut();
        result.take().unwrap().ok_or(())
    }

    fn increase_app_volume_by_percent(&mut self, index: u32, delta: f64) {
        let mut app_ref = self
            .get_app_by_index(index)
            .expect("Could not find device specified");
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = app_ref
            .volume
            .increase(new_vol)
            .expect("Volume couldn't be set");
        let op = self
            .handler
            .introspect
            .set_source_output_volume(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }

    fn decrease_app_volume_by_percent(&mut self, index: u32, delta: f64) {
        let mut app_ref = self
            .get_app_by_index(index)
            .expect("Could not find device specified");
        let new_vol = Volume::from(Volume(super::volume_from_percent(delta) as u32));
        println!("{:?}", new_vol.print_verbose(true));
        let volumes = app_ref
            .volume
            .decrease(new_vol)
            .expect("Volume couldn't be set");
        let op = self
            .handler
            .introspect
            .set_source_output_volume(index, &volumes, None);
        self.handler.wait_for_operation(op).expect("error");
    }

    fn move_app_by_index(&mut self, stream_index: u32, device_index: u32) -> Result<bool, ()> {
        let success = Rc::new(RefCell::new(false));
        let success_ref = success.clone();
        let op = self.handler.introspect.move_source_output_by_index(
            stream_index,
            device_index,
            Some(Box::new(move |res| {
                success_ref.borrow_mut().clone_from(&res)
            })),
        );
        self.handler.wait_for_operation(op)?;
        let result = success.borrow_mut().clone();
        Ok(result)
    }

    fn move_app_by_name(&mut self, stream_index: u32, device_name: &str) -> Result<bool, ()> {
        let success = Rc::new(RefCell::new(false));
        let success_ref = success.clone();
        let op = self.handler.introspect.move_source_output_by_name(
            stream_index,
            device_name,
            Some(Box::new(move |res| {
                success_ref.borrow_mut().clone_from(&res)
            })),
        );
        self.handler.wait_for_operation(op)?;
        let result = success.borrow_mut().clone();
        Ok(result)
    }

    fn set_app_mute(&mut self, index: u32, mute: bool) -> Result<bool, ()> {
        let success = Rc::new(RefCell::new(false));
        let success_ref = success.clone();
        let op = self.handler.introspect.set_source_mute_by_index(
            index,
            mute,
            Some(Box::new(move |res| {
                success_ref.borrow_mut().clone_from(&res)
            })),
        );
        self.handler.wait_for_operation(op)?;
        let result = success.borrow_mut().clone();
        Ok(result)
    }
}
