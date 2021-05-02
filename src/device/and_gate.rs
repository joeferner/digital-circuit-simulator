use crate::device;
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

impl device::Device for AndGate {
    fn next_tick(&self) -> u64 {
        self.next_tick
    }

    fn step(&mut self, _t: u64) {
        if LogicPin::is_true(self.input1) && LogicPin::is_true(self.input2) {
            self.output = LogicPin::TRUE
        } else {
            self.output = LogicPin::FALSE
        }
        self.next_tick = u64::MAX;
    }

    fn get_pin_count(&self) -> usize {
        self.pins.len()
    }

    fn get_pin(&self, i: usize) -> &dyn Pin {
        &self.pins[i - 1]
    }

    fn set_pin(&mut self, t: u64, i: usize, value: u32) {
        if i == AndGate::PIN_INPUT1 {
            self.input1 = value;
            self.next_tick = t;
        } else if i == AndGate::PIN_INPUT2 {
            self.input2 = value;
            self.next_tick = t;
        }
    }

    fn get_pin_value(&self, i: usize) -> u32 {
        if i == AndGate::PIN_INPUT1 {
            self.input1
        } else if i == AndGate::PIN_INPUT2 {
            self.input2
        } else if i == AndGate::PIN_OUTPUT {
            self.output
        } else {
            panic!("invalid pin");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::device::AndGate;
    use crate::device::Device;
    use crate::pin::LogicPin;

    #[test]
    fn it_works() {
        let mut t: u64 = 0;
        let mut gate = AndGate::new();
        assert!(LogicPin::is_false(gate.get_pin_value(AndGate::PIN_OUTPUT)));
        assert_eq!(u64::MAX, gate.next_tick());
        t += 1;

        // step no change
        gate.step(t);
        assert!(LogicPin::is_false(gate.get_pin_value(AndGate::PIN_OUTPUT)));
        assert_eq!(u64::MAX, gate.next_tick());
        t += 1;

        // set input 1 - true
        gate.set_pin(t, AndGate::PIN_INPUT1, LogicPin::TRUE);
        assert_eq!(t, gate.next_tick());
        t += 1;
        gate.step(t);
        assert!(LogicPin::is_false(gate.get_pin_value(AndGate::PIN_OUTPUT)));
        assert_eq!(u64::MAX, gate.next_tick());
        t += 1;

        // set input 2 - true
        gate.set_pin(t, AndGate::PIN_INPUT2, LogicPin::TRUE);
        assert_eq!(t, gate.next_tick());
        t += 1;
        gate.step(t);
        assert!(LogicPin::is_true(gate.get_pin_value(AndGate::PIN_OUTPUT)));
        assert_eq!(u64::MAX, gate.next_tick());
        t += 1;

        // set input 1 - false
        gate.set_pin(t, AndGate::PIN_INPUT1, LogicPin::FALSE);
        assert_eq!(t, gate.next_tick());
        t += 1;
        gate.step(t);
        assert!(LogicPin::is_false(gate.get_pin_value(AndGate::PIN_OUTPUT)));
        assert_eq!(u64::MAX, gate.next_tick());
    }
}
