use crate::pin::PinDirection;

pub trait Pin {
    fn get_pin_direction(&self) -> PinDirection;
}
