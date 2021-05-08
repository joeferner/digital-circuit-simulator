use crate::device::Device;
use crate::CircuitToDeviceMessage;
use crate::DeviceData;
use crate::DeviceToCircuitMessage;
use crate::Net;
use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Circuit {
    device_wrappers: Vec<Box<DeviceWrapper>>,
    last_tick: u64,
}

impl Circuit {
    pub fn new(devices: Vec<RefCell<Box<dyn Device>>>, _nets: Vec<Net>) -> Circuit {
        let mut device_wrappers = Vec::new();
        for device in devices {
            let (device_to_circuit_tx, device_to_circuit_rx): (
                mpsc::Sender<DeviceToCircuitMessage>,
                mpsc::Receiver<DeviceToCircuitMessage>,
            ) = mpsc::channel();
            let (circuit_to_device_tx, circuit_to_device_rx): (
                mpsc::Sender<CircuitToDeviceMessage>,
                mpsc::Receiver<CircuitToDeviceMessage>,
            ) = mpsc::channel();
            let device_name = device.borrow().get_name().to_string();
            let device_thread = thread::spawn(move || {
                device
                    .borrow_mut()
                    .run(device_to_circuit_tx, circuit_to_device_rx);
            });
            device_wrappers.push(Box::new(DeviceWrapper {
                name: device_name,
                rx: device_to_circuit_rx,
                tx: circuit_to_device_tx,
                thread: Some(device_thread),
            }));
        }

        return Circuit {
            device_wrappers,
            last_tick: 0,
        };
    }

    pub fn tick(&mut self, t: u64) -> u64 {
        if t <= self.last_tick {
            panic!("tick must be greater than last tick");
        }

        // notify devices of nex tick
        for device in &self.device_wrappers {
            device
                .tx
                .send(CircuitToDeviceMessage::NextTick { tick: t })
                .unwrap();
        }

        // wait for devices to send next tick reply
        let mut min_next_tick = u64::MAX;
        for device in &self.device_wrappers {
            match device.rx.recv() {
                Result::Ok(message) => match message {
                    DeviceToCircuitMessage::NextTick { tick } => {
                        min_next_tick = min_next_tick.min(tick);
                    }

                    DeviceToCircuitMessage::SetPin {
                        pin,
                        value,
                        direction,
                    } => {
                        println!("set pin {} {} {} {:?}", device.name, pin, value, direction);
                    }
                },
                Result::Err(_err) => {
                    panic!("failed to receive from device");
                }
            }
        }

        self.last_tick = t;
        return min_next_tick;
    }

    pub fn get_last_tick(&self) -> u64 {
        return self.last_tick;
    }

    pub fn send_device_data(&self, device_index: usize, data: Box<dyn DeviceData>) {
        self.device_wrappers[device_index]
            .tx
            .send(CircuitToDeviceMessage::Data { data })
            .unwrap();
    }
}

impl Drop for Circuit {
    fn drop(&mut self) {
        for device in &self.device_wrappers {
            device.tx.send(CircuitToDeviceMessage::Terminate).unwrap();
        }
        for device in self.device_wrappers.iter_mut() {
            device
                .thread
                .take()
                .unwrap()
                .join()
                .expect("failed to join");
        }
    }
}

#[derive(Debug)]
struct DeviceWrapper {
    name: String,
    tx: mpsc::Sender<CircuitToDeviceMessage>,
    rx: mpsc::Receiver<DeviceToCircuitMessage>,
    thread: Option<JoinHandle<()>>,
}
