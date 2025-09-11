pub(crate) const DEFAULT_DEVICE_ADDRESS: u8 = 0x27;

// /// All pins configured as inputs.
// pub const ALL_INPUTS: u8 = 0xFF;

// /// All pins configured as outputs.
// pub const ALL_OUTPUTS: u8 = 0x00;

// /// All pins normal polarity.
// pub const ALL_NORMAL_POLARITY: u8 = 0x00;

// /// All pins inverted polarity.
// pub const ALL_INVERTED_POLARITY: u8 = 0xFF;

// /// All outputs low.
// pub const ALL_OUTPUTS_LOW: u8 = 0x00;

// /// All outputs high.
// pub const ALL_OUTPUTS_HIGH: u8 = 0xFF;

pub(crate) struct Register;

impl Register {
    /// Input port register (0x00) - Read only.
    ///
    /// Reflects the incoming logic levels of the pins, regardless of whether
    /// the pin is defined as an input or an output.
    pub(crate) const INPUT: u8 = 0x00;

    /// Output port register (0x01) - Read/Write.
    ///
    /// The Output Port register shows the outgoing logic levels of the pins defined as outputs.
    pub(crate) const OUTPUT: u8 = 0x01;

    /// Polarity Inversion register (0x02) - Read/Write.
    ///
    /// This register allows the user to invert the polarity of the Input Port register data.
    pub(crate) const POLARITY: u8 = 0x02;

    /// Configuration register (0x03) - Read/Write.
    ///
    /// The Configuration register configures the directions of the I/O pins.
    ///
    /// - 1 = pin is configured as an input (default);
    /// - 0 = pin is configured as an output.
    pub(crate) const CONFIG: u8 = 0x03;
}
