use crate::pin::Pin;
use core::fmt::Debug;

pub trait Device {
    fn next_tick(&self) -> u64;

    fn step(&mut self, t: u64);

    fn get_pin_count(&self) -> usize;

    /// Returns the pin at offset i (one based)
    ///
    /// # Arguments
    ///
    /// * `i` - one based index into pins
    fn get_pin(&self, i: usize) -> &dyn Pin;
}

impl Debug for dyn Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Device").finish()
    }
}
