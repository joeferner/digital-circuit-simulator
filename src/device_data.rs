use std::any::Any;

pub trait DeviceData: Send {
    fn as_any(&self) -> &dyn Any;
}
