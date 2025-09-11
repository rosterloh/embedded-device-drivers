/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided.
    InvalidInputData,
}

/// Pin configuration (direction).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PinConfig {
    /// Pin configured as input (high impedance) - default.
    Input = 1,
    /// Pin configured as output (can drive high or low).
    Output = 0,
}

impl PinConfig {
    /// Get pin config bit value.
    pub fn bits(self) -> u8 {
        self as u8
    }
}

/// Pin polarity setting.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PinPolarity {
    /// Normal polarity (default) - GPIO register bit reflects same value on the input pin.
    Normal = 0,
    /// Inverted polarity - GPIO register bit reflects inverted value on the input pin.
    Inverted = 1,
}

impl PinPolarity {
    /// Get polarity bit value.
    pub fn bits(self) -> u8 {
        self as u8
    }
}

/// Pin logic level.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PinLevel {
    /// Logic low (0V).
    Low = 0,
    /// Logic high (VCC).
    High = 1,
}

impl PinLevel {
    /// Get level bit value.
    pub fn bits(self) -> u8 {
        self as u8
    }
}

// /// Configuration constants.
// pub mod config {
//     /// All pins configured as inputs.
//     pub const ALL_INPUTS: u8 = 0xFF;

//     /// All pins configured as outputs.
//     pub const ALL_OUTPUTS: u8 = 0x00;

//     /// All pins normal polarity.
//     pub const ALL_NORMAL_POLARITY: u8 = 0x00;

//     /// All pins inverted polarity.
//     pub const ALL_INVERTED_POLARITY: u8 = 0xFF;

//     /// All outputs low.
//     pub const ALL_OUTPUTS_LOW: u8 = 0x00;

//     /// All outputs high.
//     pub const ALL_OUTPUTS_HIGH: u8 = 0xFF;
// }

// /// Common I2C addresses for TCA9534 based on A2, A1, A0 pins.
// pub mod addresses {
//     /// A2=0, A1=0, A0=0 (default).
//     pub const ADDR_000: u8 = 0x20;
//     /// A2=0, A1=0, A0=1.
//     pub const ADDR_001: u8 = 0x21;
//     /// A2=0, A1=1, A0=0.
//     pub const ADDR_010: u8 = 0x22;
//     /// A2=0, A1=1, A0=1.
//     pub const ADDR_011: u8 = 0x23;
//     /// A2=1, A1=0, A0=0.
//     pub const ADDR_100: u8 = 0x24;
//     /// A2=1, A1=0, A0=1.
//     pub const ADDR_101: u8 = 0x25;
//     /// A2=1, A1=1, A0=0.
//     pub const ADDR_110: u8 = 0x26;
//     /// A2=1, A1=1, A0=1.
//     pub const ADDR_111: u8 = 0x27;
// }
