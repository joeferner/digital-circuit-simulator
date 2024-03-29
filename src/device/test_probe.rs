use crate::device::Device;
use crate::Circuit;
use crate::CircuitToDeviceMessage;
use crate::DeviceData;
use crate::DeviceToCircuitMessage;
use crate::PinDirection;
use std::any::Any;
use std::sync::mpsc;

#[derive(Debug)]
pub struct TestProbe {
    name: String,
    value: u32,
    direction: PinDirection,
    dirty: bool,
}

impl TestProbe {
    pub const PIN: usize = 1;

    pub fn new(name: &str, value: u32, direction: PinDirection) -> TestProbe {
        TestProbe {
            name: name.to_string(),
            value,
            direction,
            dirty: true,
        }
    }

    pub fn set_output_high(circuit: &Circuit, device: usize) {
        circuit.send_device_data(device, Box::new(TestProbeSetData::output_high()));
    }

    pub fn set_output_low(circuit: &Circuit, device: usize) {
        circuit.send_device_data(device, Box::new(TestProbeSetData::output_low()));
    }

    pub fn set_input(circuit: &Circuit, device: usize) {
        circuit.send_device_data(device, Box::new(TestProbeSetData::input()));
    }

    pub fn get_value(circuit: &Circuit, device: usize) -> u32 {
        let results = circuit.recv_device_data(device, Box::new(TestProbeGetDataRequest::new()));
        let data = results
            .as_any()
            .downcast_ref::<TestProbeGetDataResponse>()
            .unwrap();
        return data.get_value();
    }
}

impl Device for TestProbe {
    fn run(
        &mut self,
        tx: mpsc::Sender<DeviceToCircuitMessage>,
        rx: mpsc::Receiver<CircuitToDeviceMessage>,
    ) {
        let mut run = true;
        while run {
            match rx.recv() {
                Result::Ok(message) => match message {
                    CircuitToDeviceMessage::NextTick { tick: _ } => {
                        if self.dirty {
                            tx.send(DeviceToCircuitMessage::SetPin {
                                pin: TestProbe::PIN,
                                value: self.value,
                                direction: self.direction,
                            })
                            .unwrap();
                            self.dirty = false;
                        }
                        tx.send(DeviceToCircuitMessage::NextTick { tick: u64::MAX })
                            .unwrap();
                    }
                    CircuitToDeviceMessage::SetPin {
                        tick: _,
                        pin,
                        value,
                        last,
                    } => match self.direction {
                        PinDirection::Input => {
                            if pin == TestProbe::PIN {
                                self.value = value;
                            } else {
                                panic!("cannot set pin {} on test probe", pin);
                            }
                            if last {
                                tx.send(DeviceToCircuitMessage::NextTick { tick: u64::MAX })
                                    .unwrap();
                            }
                        }
                        PinDirection::Output => {
                            panic!("invalid set pin");
                        }
                    },
                    CircuitToDeviceMessage::Terminate => {
                        run = false;
                    }
                    CircuitToDeviceMessage::Data { data } => {
                        if let Some(set_data) = data.as_any().downcast_ref::<TestProbeSetData>() {
                            self.value = set_data.get_value();
                            self.direction = set_data.get_direction();
                            self.dirty = true;
                        } else if let Some(_get_data) =
                            data.as_any().downcast_ref::<TestProbeGetDataRequest>()
                        {
                            tx.send(DeviceToCircuitMessage::Data {
                                data: Box::new(TestProbeGetDataResponse::new(self.value)),
                            })
                            .unwrap();
                        } else {
                            panic!("unexpected data");
                        }
                    }
                },
                Result::Err(_err) => {
                    run = false;
                }
            }
        }
    }

    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_pin_count(&self) -> usize {
        return 1;
    }
}

#[derive(Debug)]
pub struct TestProbeSetData {
    value: u32,
    direction: PinDirection,
}

impl TestProbeSetData {
    pub fn new(value: u32, direction: PinDirection) -> TestProbeSetData {
        return TestProbeSetData { value, direction };
    }

    pub fn output_high() -> TestProbeSetData {
        return TestProbeSetData::new(u32::MAX, PinDirection::Output);
    }

    pub fn output_low() -> TestProbeSetData {
        return TestProbeSetData::new(0, PinDirection::Output);
    }

    pub fn input() -> TestProbeSetData {
        return TestProbeSetData::new(0, PinDirection::Input);
    }

    pub fn get_value(&self) -> u32 {
        return self.value;
    }

    pub fn get_direction(&self) -> PinDirection {
        return self.direction;
    }
}

impl DeviceData for TestProbeSetData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct TestProbeGetDataRequest {}

impl TestProbeGetDataRequest {
    pub fn new() -> TestProbeGetDataRequest {
        return TestProbeGetDataRequest {};
    }
}

impl DeviceData for TestProbeGetDataRequest {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct TestProbeGetDataResponse {
    value: u32,
}

impl TestProbeGetDataResponse {
    pub fn new(value: u32) -> TestProbeGetDataResponse {
        return TestProbeGetDataResponse { value };
    }

    pub fn get_value(&self) -> u32 {
        return self.value;
    }
}

impl DeviceData for TestProbeGetDataResponse {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
