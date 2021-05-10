use crate::DeviceData;
use crate::PinDirection;

pub enum DeviceToCircuitMessage {
    NextTick {
        tick: u64,
    },
    SetPin {
        pin: usize,
        value: u32,
        direction: PinDirection,
    },
    Data {
        data: Box<dyn DeviceData>,
    },
}

pub enum CircuitToDeviceMessage {
    Data {
        data: Box<dyn DeviceData>,
    },
    NextTick {
        tick: u64,
    },
    SetPin {
        tick: u64,
        pin: usize,
        value: u32,
        last: bool,
    },
    Terminate,
}
