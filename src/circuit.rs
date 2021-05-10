use crate::device::Device;
use crate::CircuitToDeviceMessage;
use crate::DeviceData;
use crate::DeviceToCircuitMessage;
use crate::Net;
use crate::PinDirection;
use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Circuit {
    device_wrappers: Vec<Box<DeviceWrapper>>,
    last_tick: u64,
    // nets[device_index][pin_index] = Vec<> of connected pins
    nets: Vec<Vec<Vec<PinRef>>>,
}

impl Circuit {
    pub fn new(devices: Vec<RefCell<Box<dyn Device>>>, nets: Vec<Net>) -> Circuit {
        let mut circuit_nets: Vec<Vec<Vec<PinRef>>> = Vec::new();
        let mut device_wrappers: Vec<Box<DeviceWrapper>> = Vec::new();
        for device in devices {
            let device_index = circuit_nets.len();
            let mut device_nets: Vec<Vec<PinRef>> = Vec::new();
            for _i in 0..(device.borrow().get_pin_count() + 1) {
                device_nets.push(Vec::new());
            }
            circuit_nets.push(device_nets);

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
                index: device_index,
                name: device_name,
                rx: device_to_circuit_rx,
                tx: circuit_to_device_tx,
                thread: Some(device_thread),
            }));
        }

        for net in nets {
            for from_conn in net.connections_iter() {
                let from_device = from_conn.get_device();
                let from_pin = from_conn.get_pin();
                for to_conn in net.connections_iter() {
                    let to_device = to_conn.get_device();
                    let to_pin = to_conn.get_pin();
                    if from_device != to_device || from_pin != to_pin {
                        circuit_nets[from_device][from_pin].push(PinRef {
                            device: to_device,
                            pin: to_pin,
                        });
                    }
                }
            }
        }
        return Circuit {
            device_wrappers,
            last_tick: 0,
            nets: circuit_nets,
        };
    }

    pub fn tick(&mut self, tick: u64) -> u64 {
        if tick <= self.last_tick {
            panic!("tick must be greater than last tick");
        }

        // notify devices of nex tick
        for device in &self.device_wrappers {
            device
                .tx
                .send(CircuitToDeviceMessage::NextTick { tick: tick })
                .unwrap();
        }

        // wait for devices to send next tick reply
        let mut min_next_tick = u64::MAX;
        for device in &self.device_wrappers {
            let mut rx_next_tick = false;
            while !rx_next_tick {
                match device.rx.recv() {
                    Result::Ok(message) => match message {
                        DeviceToCircuitMessage::NextTick { tick } => {
                            min_next_tick = min_next_tick.min(tick);
                            rx_next_tick = true;
                        }

                        DeviceToCircuitMessage::SetPin {
                            pin,
                            value,
                            direction,
                        } => {
                            self.process_set_pin(tick, device, pin, value, direction);
                        }

                        DeviceToCircuitMessage::Data { data: _ } => {
                            panic!("unexpected data");
                        }
                    },
                    Result::Err(_err) => {
                        panic!("failed to receive from device");
                    }
                }
            }
        }

        self.last_tick = tick;
        return min_next_tick;
    }

    fn process_set_pin(
        &self,
        tick: u64,
        device: &DeviceWrapper,
        pin: usize,
        value: u32,
        direction: PinDirection,
    ) {
        match direction {
            PinDirection::Output => {
                let connections: &Vec<PinRef> = &self.nets[device.index][pin];
                for connection in connections.iter() {
                    self.device_wrappers[connection.device]
                        .tx
                        .send(CircuitToDeviceMessage::SetPin {
                            tick,
                            pin: connection.pin,
                            value: value,
                            last: true,
                        })
                        .unwrap();
                }
            }
            PinDirection::Input => (),
        }
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

    pub fn recv_device_data(
        &self,
        device_index: usize,
        data: Box<dyn DeviceData>,
    ) -> Box<dyn DeviceData> {
        self.send_device_data(device_index, data);
        let results = self.device_wrappers[device_index].rx.recv();
        match results {
            Result::Ok(message) => match message {
                DeviceToCircuitMessage::Data { data } => {
                    return data;
                }

                _ => {
                    panic!("unexpected data response");
                }
            },
            Result::Err(_err) => {
                panic!("failed to receive from device");
            }
        }
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
    index: usize,
    name: String,
    tx: mpsc::Sender<CircuitToDeviceMessage>,
    rx: mpsc::Receiver<DeviceToCircuitMessage>,
    thread: Option<JoinHandle<()>>,
}

#[derive(Debug)]
struct PinRef {
    device: usize,
    pin: usize,
}
