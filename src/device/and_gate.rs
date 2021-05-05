use crate::device::Device;
use crate::pin::LogicPin;
use crate::pin::Pin;
use crate::pin::PinDirection;

#[derive(Debug)]
pub struct AndGate {
    pins: [LogicPin; 3],
    input1: u32,
    input2: u32,
    output: u32,
    next_tick: u64,
}

impl AndGate {
    pub const PIN_INPUT1: usize = 1;
    pub const PIN_INPUT2: usize = 2;
    pub const PIN_OUTPUT: usize = 3;

    pub fn new() -> AndGate {
        AndGate {
            pins: [
                LogicPin::new(PinDirection::Input),
                LogicPin::new(PinDirection::Input),
                LogicPin::new(PinDirection::Output),
            ],
            input1: LogicPin::FALSE,
            input2: LogicPin::FALSE,
            output: LogicPin::FALSE,
            next_tick: u64::MAX,
        }
    }
}

impl Device for AndGate {
    fn next_tick(&self) -> u64 {
        self.next_tick
    }

    fn step(&mut self, _t: u64) {
        // TODO tell circuit pin changed
        if LogicPin::is_true(self.input1) && LogicPin::is_true(self.input2) {
            self.output = LogicPin::TRUE
        } else {
            self.output = LogicPin::FALSE
        }
        self.next_tick = u64::MAX;
    }

    fn get_pin_count(&self) -> usize {
        return self.pins.len();
    }

    fn get_pin(&self, i: usize) -> &dyn Pin {
        return &self.pins[i - 1];
    }
}

#[cfg(test)]
mod tests {
    use crate::device::AndGate;
    use crate::device::Device;
    use crate::pin::LogicPin;
    use crate::Circuit;
    use crate::Net;
    use crate::NetConnection;

    #[test]
    fn it_works() {
        const DEVICE_AND_GATE: u32 = 0;
        let and_gate = AndGate::new();
        let devices: Vec<Box<dyn Device>> = vec![Box::new(and_gate)];
        let net0 = Net::new(vec![NetConnection::new(
            DEVICE_AND_GATE,
            AndGate::PIN_INPUT1,
        )]);
        let net1 = Net::new(vec![NetConnection::new(
            DEVICE_AND_GATE,
            AndGate::PIN_INPUT2,
        )]);
        let net2 = Net::new(vec![NetConnection::new(
            DEVICE_AND_GATE,
            AndGate::PIN_OUTPUT,
        )]);
        let nets = vec![net0, net1, net2];
        let mut circuit = Circuit::new(devices, nets);

        circuit.tick(1);
        assert_eq!(
            LogicPin::FALSE,
            circuit.get_device_pin_value(DEVICE_AND_GATE, AndGate::PIN_OUTPUT)
        );

        circuit.set_device_pin_value(DEVICE_AND_GATE, AndGate::PIN_INPUT1, LogicPin::TRUE);
        circuit.tick(2);
        assert_eq!(
            LogicPin::FALSE,
            circuit.get_device_pin_value(DEVICE_AND_GATE, AndGate::PIN_OUTPUT)
        );

        circuit.set_device_pin_value(DEVICE_AND_GATE, AndGate::PIN_INPUT2, LogicPin::TRUE);
        circuit.tick(3);
        assert_eq!(
            LogicPin::TRUE,
            circuit.get_device_pin_value(DEVICE_AND_GATE, AndGate::PIN_OUTPUT)
        );

        circuit.set_device_pin_value(DEVICE_AND_GATE, AndGate::PIN_INPUT1, LogicPin::FALSE);
        circuit.tick(4);
        assert_eq!(
            LogicPin::FALSE,
            circuit.get_device_pin_value(DEVICE_AND_GATE, AndGate::PIN_OUTPUT)
        );
    }
}
