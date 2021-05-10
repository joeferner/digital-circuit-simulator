use crate::device::Device;
use crate::CircuitToDeviceMessage;
use crate::DeviceToCircuitMessage;
use crate::PinDirection;
use std::sync::mpsc;

#[derive(Debug)]
pub struct AndGate {
    name: String,
    input1: bool,
    input2: bool,
    last: bool,
}

impl AndGate {
    pub const PIN_INPUT1: usize = 1;
    pub const PIN_INPUT2: usize = 2;
    pub const PIN_OUTPUT: usize = 3;

    pub fn new(name: &str) -> AndGate {
        AndGate {
            name: name.to_string(),
            input1: false,
            input2: false,
            last: false,
        }
    }
}

impl Device for AndGate {
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
                        let new_result = self.input1 && self.input2;
                        if new_result != self.last {
                            tx.send(DeviceToCircuitMessage::SetPin {
                                pin: AndGate::PIN_OUTPUT,
                                value: u32::MAX,
                                direction: PinDirection::Output,
                            })
                            .unwrap();
                        }
                        tx.send(DeviceToCircuitMessage::NextTick { tick: u64::MAX })
                            .unwrap();
                    }
                    CircuitToDeviceMessage::Terminate => {
                        run = false;
                    }
                    CircuitToDeviceMessage::SetPin {
                        tick,
                        pin,
                        value,
                        last,
                    } => {
                        if pin == AndGate::PIN_INPUT1 {
                            self.input1 = value > 0;
                        } else if pin == AndGate::PIN_INPUT2 {
                            self.input2 = value > 0;
                        } else {
                            panic!("cannot set pin {} on and gate", pin);
                        }
                        if last {
                            tx.send(DeviceToCircuitMessage::NextTick { tick: tick + 1 })
                                .unwrap();
                        }
                    }
                    CircuitToDeviceMessage::Data { data: _ } => {
                        panic!("not expecting data");
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
        return 3;
    }
}

#[cfg(test)]
mod tests {
    use crate::device::AndGate;
    use crate::device::Device;
    use crate::device::TestProbe;
    use crate::Circuit;
    use crate::Net;
    use crate::NetConnection;
    use crate::PinDirection;
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        const DEVICE_AND_GATE: usize = 0;
        const DEVICE_INPUT1: usize = 1;
        const DEVICE_INPUT2: usize = 2;
        const DEVICE_OUTPUT: usize = 3;
        let and_gate = AndGate::new("and");
        let input1_tp = TestProbe::new("input1_tp", 0, PinDirection::Output);
        let input2_tp = TestProbe::new("input2_tp", 0, PinDirection::Output);
        let output_tp = TestProbe::new("output_tp", 0, PinDirection::Input);
        let devices: Vec<RefCell<Box<dyn Device>>> = vec![
            RefCell::new(Box::new(and_gate)),
            RefCell::new(Box::new(input1_tp)),
            RefCell::new(Box::new(input2_tp)),
            RefCell::new(Box::new(output_tp)),
        ];
        let net0 = Net::new(vec![
            NetConnection::new(DEVICE_AND_GATE, AndGate::PIN_INPUT1),
            NetConnection::new(DEVICE_INPUT1, TestProbe::PIN),
        ]);
        let net1 = Net::new(vec![
            NetConnection::new(DEVICE_AND_GATE, AndGate::PIN_INPUT2),
            NetConnection::new(DEVICE_INPUT2, TestProbe::PIN),
        ]);
        let net2 = Net::new(vec![
            NetConnection::new(DEVICE_AND_GATE, AndGate::PIN_OUTPUT),
            NetConnection::new(DEVICE_OUTPUT, TestProbe::PIN),
        ]);
        let nets = vec![net0, net1, net2];
        let mut circuit = Circuit::new(devices, nets);
        let mut next_tick = circuit.tick(1);
        assert_eq!(u64::MAX, next_tick);

        TestProbe::set_output_high(&circuit, DEVICE_INPUT1);
        next_tick = circuit.tick(2);
        assert_eq!(u64::MAX, next_tick);
        assert_eq!(0, TestProbe::get_value(&circuit, DEVICE_OUTPUT));

        TestProbe::set_output_high(&circuit, DEVICE_INPUT2);
        next_tick = circuit.tick(3);
        assert_eq!(u64::MAX, next_tick);
        assert_eq!(u32::MAX, TestProbe::get_value(&circuit, DEVICE_OUTPUT));
    }
}
