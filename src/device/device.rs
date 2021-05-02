use crate::pin::Pin;

pub trait Device {
    fn next_tick() -> u64;

    fn step(&self, t: u64);

    fn get_pin_count(&self) -> usize;

    /// Returns the pin at offset i (one based)
    /// 
    /// # Arguments
    /// 
    /// * `i` - one based index into pins
    fn get_pin(&self, i: usize) -> &dyn Pin;
}
