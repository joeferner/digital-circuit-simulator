use crate::pin::Pin;
use crate::pin::PinDirection;

#[derive(Debug)]
pub struct LogicPin {
    direction: PinDirection,
}

impl LogicPin {
    pub const TRUE: u32 = u32::MAX;
    pub const FALSE: u32 = 0;

    pub fn is_false(value: u32) -> bool {
        value == LogicPin::FALSE
    }

    pub fn is_true(value: u32) -> bool {
        value != LogicPin::FALSE
    }

    pub fn new(direction: PinDirection) -> LogicPin {
        LogicPin {
            direction: direction,
        }
    }
}

impl Pin for LogicPin {
    fn get_pin_direction(&self) -> PinDirection {
        self.direction
    }
}
