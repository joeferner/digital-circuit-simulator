use crate::device;
use crate::pin::LogicPin;
use crate::pin::Pin;
use crate::pin::PinDirection;

#[derive(Debug)]
pub struct AndGate {
    pins: [LogicPin; 3],
}

impl AndGate {
    pub fn new() -> AndGate {
        AndGate {
            pins: [
                LogicPin::new(PinDirection::Input),
                LogicPin::new(PinDirection::Input),
                LogicPin::new(PinDirection::Output),
            ],
        }
    }
}

impl device::Device for AndGate {
    fn next_tick() -> u64 {
        std::u64::MAX
    }

    fn step(&self, _t: u64) {
        todo!()
    }

    fn get_pin_count(&self) -> usize {
        self.pins.len()
    }

    fn get_pin(&self, i: usize) -> &dyn Pin {
        &self.pins[i - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::device::AndGate;

    #[test]
    fn it_works() {
        let gate = AndGate::new();
        println!("and gate {:?}", gate);
    }
}
