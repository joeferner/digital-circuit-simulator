use crate::device::Device;
use crate::Net;

#[derive(Debug)]
pub struct Circuit {
    devices: Vec<Box<dyn Device>>,
    nets: Vec<Net>,
}

impl Circuit {
    pub fn new(devices: Vec<Box<dyn Device>>, nets: Vec<Net>) -> Circuit {
        Circuit { devices, nets }
    }

    pub fn tick(&mut self, t: u64) {
        println!("{}", t);
        for device in self.devices.iter() {
            println!("{:?}", device);
        }
    }

    pub fn get_device_pin_value(&self, device: u32, pin: usize) -> u32 {
        match self.get_net_for_device_and_pin(device, pin) {
            Some(x) => return x.get_value(),
            None => panic!("invalid device {} or pin {}", device, pin),
        }
    }

    pub fn set_device_pin_value(&mut self, device: u32, pin: usize, value: u32) {
        match self.get_net_for_device_and_pin_mut(device, pin) {
            Some(x) => return x.set_value(value),
            None => panic!("invalid device {} or pin {}", device, pin),
        }
    }

    pub fn get_net_for_device_and_pin(&self, device: u32, pin: usize) -> Option<&Net> {
        for net in self.nets.iter() {
            for connection in net.connections_iter() {
                if connection.get_device() == device && connection.get_pin() == pin {
                    return Some(net);
                }
            }
        }
        return None;
    }

    pub fn get_net_for_device_and_pin_mut(&mut self, device: u32, pin: usize) -> Option<&mut Net> {
        for net in self.nets.iter_mut() {
            for connection in net.connections_iter() {
                if connection.get_device() == device && connection.get_pin() == pin {
                    return Some(net);
                }
            }
        }
        return None;
    }
}
