use crate::pin::Pin;
use crate::pin::PinDirection;

#[derive(Debug)]
pub struct LogicPin {
    direction: PinDirection,
}

impl LogicPin {
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
