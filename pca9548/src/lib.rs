//! This is a platform agnostic Rust driver for the TCA9548A 8-Channel I2C
//! Multiplexer, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! Datasheet:
//!  - [Adafruit](https://learn.adafruit.com/adafruit-tca9548a-1-to-8-i2c-multiplexer-breakout/overview)
//!  - [TCA9548A](https://www.ti.com/lit/ds/symlink/tca9548a.pdf)
//!
//! ## Usage
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate the device
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pca9548::prelude::*;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut mux = PCA9548::new(dev).with_ports_disabled().unwrap();
//! // Enable port 0
//! mux.set_port(0, true).unwrap();
//! ```

#![deny(unsafe_code)]
#![no_std]

#[cfg(feature = "bus")]
pub mod bus;
pub mod device;
pub mod error;

pub mod prelude {
    #[cfg(feature = "bus")]
    pub use crate::bus::{BusPort, MultiplexerBus};
    pub use crate::{PCA9548, PortState, error::PCA9548Error};
}

const DEFAULT_DEVICE_ADDRESS: u8 = 0x70;
const MAX_PORTS: usize = 8;

#[derive(Copy, Clone, Debug)]
pub enum PortState {
    Enabled,
    Disabled,
}

impl From<bool> for PortState {
    fn from(value: bool) -> Self {
        match value {
            true => PortState::Enabled,
            false => PortState::Disabled,
        }
    }
}

#[derive(Debug)]
pub struct PCA9548<I2C> {
    i2c: I2C,
    address: u8,
    state: [bool; MAX_PORTS],
}

pub(crate) fn address_from_pins(a0: bool, a1: bool, a2: bool) -> u8 {
    let mut address = 0b1110_0000;
    if a0 {
        address |= 0b0000_0001;
    }
    if a1 {
        address |= 0b0000_0010;
    }
    if a2 {
        address |= 0b0000_0100;
    }
    address
}

impl<I2C> PCA9548<I2C> {
    /// Create new instance of a PCA9548 device.
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            address: DEFAULT_DEVICE_ADDRESS,
            state: [false; MAX_PORTS],
        }
    }

    /// Sets the address according to the enabled hardware settings
    pub fn with_address_pins(mut self, a0: bool, a1: bool, a2: bool) -> Self {
        self.address = address_from_pins(a0, a1, a2);
        self
    }

    /// Create new instance of the PCA9548 device specifying the address.
    pub fn with_address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    fn port_code(states: [bool; MAX_PORTS]) -> u8 {
        let mut code = 0;
        if states[0] {
            code |= 0b0000_0001;
        }
        if states[1] {
            code |= 0b0000_0010;
        }
        if states[2] {
            code |= 0b0000_0100;
        }
        if states[3] {
            code |= 0b0000_1000;
        }
        if states[4] {
            code |= 0b0001_0000;
        }
        if states[5] {
            code |= 0b0010_0000;
        }
        if states[6] {
            code |= 0b0100_0000;
        }
        if states[7] {
            code |= 0b1000_0000;
        }

        code
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
