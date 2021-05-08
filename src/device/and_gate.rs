use crate::device::Device;
use crate::CircuitToDeviceMessage;
use crate::DeviceToCircuitMessage;
use std::sync::mpsc;

#[derive(Debug)]
pub struct AndGate {
    name: String,
    input1: bool,
    input2: bool,
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
                        tx.send(DeviceToCircuitMessage::NextTick { tick: u64::MAX })
                            .unwrap();
                    }
                    CircuitToDeviceMessage::Terminate => {
                        run = false;
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
}

#[cfg(test)]
mod tests {
    use crate::device::AndGate;
    use crate::device::Device;
    use crate::device::PinDirection;
    use crate::device::TestProbe;
    use crate::device::TestProbeSetData;
    use crate::Circuit;
    use crate::Net;
    use crate::NetConnection;
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        const DEVICE_AND_GATE: usize = 0;
        const DEVICE_INPUT1: usize = 1;
        const DEVICE_INPUT2: usize = 2;
        let and_gate = AndGate::new("and");
        let input1_source = TestProbe::new("input1", 0, PinDirection::Output);
        let input2_source = TestProbe::new("input2", 0, PinDirection::Output);
        let devices: Vec<RefCell<Box<dyn Device>>> = vec![
            RefCell::new(Box::new(and_gate)),
            RefCell::new(Box::new(input1_source)),
            RefCell::new(Box::new(input2_source)),
        ];
        let net0 = Net::new(vec![
            NetConnection::new(DEVICE_AND_GATE, AndGate::PIN_INPUT1),
            NetConnection::new(DEVICE_INPUT1, TestProbe::PIN),
        ]);
        let net1 = Net::new(vec![
            NetConnection::new(DEVICE_AND_GATE, AndGate::PIN_INPUT2),
            NetConnection::new(DEVICE_INPUT2, TestProbe::PIN),
        ]);
        let net2 = Net::new(vec![NetConnection::new(
            DEVICE_AND_GATE,
            AndGate::PIN_OUTPUT,
        )]);
        let nets = vec![net0, net1, net2];
        let mut circuit = Circuit::new(devices, nets);
        let mut next_tick = 1;

        next_tick = circuit.tick(next_tick);
        assert_eq!(u64::MAX, next_tick);

        circuit.send_device_data(DEVICE_INPUT1, Box::new(TestProbeSetData::output_high()));
        next_tick = 2;
        next_tick = circuit.tick(next_tick);
        assert_eq!(u64::MAX, next_tick);
    }
}
