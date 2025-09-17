//! This is a platform agnostic Rust driver for the Sparkfun Qwiic GPIO
//! Expander, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! Datasheet:
//!  - [Sparkfun](https://www.sparkfun.com/sparkfun-qwiic-gpio.html)
//!  - [TCA9534](https://www.ti.com/lit/ds/symlink/tca9534.pdf)
//!
//! ## Usage
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate the device
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use sparkfun_qwiic_gpio::{PinConfig, PinLevel, SparkfunQwiicGpio};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut gpio = SparkfunQwiicGpio::new(dev);
//! gpio.init().unwrap();
//! // Configure pin 0 as output, others as input
//! gpio.set_pin_config(0, PinConfig::Output).unwrap();
//! gpio.set_pin_config(1, PinConfig::Input).unwrap();
//! // Set pin 0 to high
//! gpio.set_pin_output(0, PinLevel::High).unwrap();
//! // Read pin 1 input
//! let pin1_level = gpio.read_pin_input(1).unwrap();
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod device;
mod interface;
use crate::interface::{Register, DEFAULT_DEVICE_ADDRESS};
pub use crate::interface::{
    ALL_INPUTS, ALL_INVERTED_POLARITY, ALL_NORMAL_POLARITY, ALL_OUTPUTS, ALL_OUTPUTS_HIGH,
    ALL_OUTPUTS_LOW,
};
mod types;
pub use crate::types::{Error, PinConfig, PinLevel, PinPolarity};

/// Sparkfun Qwiic GPIO driver structure.
#[derive(Debug)]
pub struct SparkfunQwiicGpio<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> SparkfunQwiicGpio<I2C> {
    /// Create new instance of the SparkfunQwiicGpio device.
    pub fn new(i2c: I2C) -> Self {
        SparkfunQwiicGpio {
            i2c,
            address: DEFAULT_DEVICE_ADDRESS,
        }
    }

    /// Create new instance of the SparkfunQwiicGpio device.
    pub fn new_with_address(i2c: I2C, address: u8) -> Self {
        SparkfunQwiicGpio { i2c, address }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
