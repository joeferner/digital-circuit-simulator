use crate::CircuitToDeviceMessage;
use crate::DeviceToCircuitMessage;
use core::fmt::Debug;
use std::sync::mpsc;

pub trait Device: Send {
    fn run(
        &mut self,
        tx: mpsc::Sender<DeviceToCircuitMessage>,
        rx: mpsc::Receiver<CircuitToDeviceMessage>,
    );

    fn get_name(&self) -> &str;

    /// Gets the number of pins
    fn get_pin_count(&self) -> usize;
}

impl Debug for dyn Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Device").finish()
    }
}
