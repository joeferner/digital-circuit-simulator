use crate::device::PinDirection;
use crate::DeviceData;

pub enum DeviceToCircuitMessage {
    NextTick {
        tick: u64,
    },
    SetPin {
        pin: usize,
        value: u32,
        direction: PinDirection,
    },
}

pub enum CircuitToDeviceMessage {
    Data { data: Box<dyn DeviceData> },
    NextTick { tick: u64 },
    Terminate,
}
