use crate::device::Device;
use crate::device::PinDirection;
use crate::CircuitToDeviceMessage;
use crate::DeviceData;
use crate::DeviceToCircuitMessage;
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
                    CircuitToDeviceMessage::Terminate => {
                        run = false;
                    }
                    CircuitToDeviceMessage::Data { data } => {
                        let data = data.as_any().downcast_ref::<TestProbeSetData>().unwrap();
                        self.value = data.get_value();
                        self.direction = data.get_direction();
                        self.dirty = true;
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
