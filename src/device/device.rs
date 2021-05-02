use crate::pin::Pin;

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

    fn set_pin(&mut self, t: u64, i: usize, value: u32);

    fn get_pin_value(&self, i: usize) -> u32;
}
