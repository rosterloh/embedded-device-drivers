/// Default I2C address for the device.
pub const DEFAULT_DEVICE_ADDRESS: u8 = 0x04;

pub(crate) const READ_COMMAND: u8 = 0x01;
pub(crate) const WRITE_COMMAND: u8 = 0x02;
pub(crate) const END_OF_PACKET: u8 = 0xFF;

pub(crate) struct Register;

#[allow(dead_code)]
impl Register {
    pub(crate) const ADDRESS: u8 = 0x00;
    pub(crate) const SERIAL: u8 = 0x01;
    pub(crate) const ACCUMULATOR: u8 = 0x05;
    pub(crate) const GAIN: u8 = 0x06;
    pub(crate) const FIRMWARE: u8 = 0x07;
    pub(crate) const DISCHARGE_TIME: u8 = 0x08;
    pub(crate) const OUTPUT_CURRENT: u8 = 0x09;
    pub(crate) const OUTPUT_SCALE: u8 = 0x0A;
    pub(crate) const NUM_ELEMENTS: u8 = 0x0C;
    pub(crate) const CALIBRATED: u8 = 0x0D;
    pub(crate) const BASELINE: u8 = 0x29;
    pub(crate) const FRAME_IDX: u8 = 0x80;
    pub(crate) const TIMESTAMP: u8 = 0x82;
    pub(crate) const OUTPUT_DATA: u8 = 0x84;
}

// pub(crate) struct BitFlags;

// impl BitFlags {
//     pub(crate) const CMD: u8 = 0b1000_0000;
//     pub(crate) const CMD_AUTO_INC: u8 = 0b0010_0000;
//     pub(crate) const POWER_ON: u8 = 0b0000_0001; // PON
//     pub(crate) const RGBC_EN: u8 = 0b0000_0010; // AEN
//     pub(crate) const WAIT_EN: u8 = 0b0000_1000; // WEN
//     pub(crate) const RGBC_INT_EN: u8 = 0b0001_0000; // AIEN
//     pub(crate) const RGBC_VALID: u8 = 0b0000_0001; // AVALID
//     pub(crate) const WLONG: u8 = 0b0000_0010;
// }
