use std::any::Any;

pub trait DeviceData: Send + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}
