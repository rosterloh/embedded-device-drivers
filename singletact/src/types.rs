/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided.
    InvalidInputData,
}

/// Result of a measurement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SensorFrameMeasurement {
    /// Frame index.
    pub index: u16,
    /// Sensor timestamp (0.1 ms increments).
    pub timestamp: u16,
    /// Sensor output (10 bit raw count).
    pub output: u16,
}
