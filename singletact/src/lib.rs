//! This is a platform agnostic Rust driver for the SingleTact force
//! sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! Links:
//! - [Datasheet](https://5361756.fs1.hubspotusercontent-na1.net/hubfs/5361756/SingleTact%20Documents/SingleTact_Datasheet.pdf)
//! - [Manual](https://5361756.fs1.hubspotusercontent-na1.net/hubfs/5361756/SingleTact%20Documents/SingleTact_Manual.pdf)

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod device;
mod interface;
use crate::interface::{END_OF_PACKET, READ_COMMAND, Register, WRITE_COMMAND};
mod types;
pub use crate::types::{Error, SensorFrameMeasurement};

/// SingleTact device driver.
#[derive(Debug)]
pub struct SingleTact<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Address of the device on the bus.
    address: u8,
}

impl<I2C> SingleTact<I2C> {
    /// Create new instance of the SingleTact device.
    pub fn new(i2c: I2C, address: u8) -> Self {
        SingleTact { i2c, address }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
