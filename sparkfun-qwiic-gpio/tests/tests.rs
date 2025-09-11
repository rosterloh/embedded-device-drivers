use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use sparkfun_qwiic_gpio::SparkfunQwiicGpio;

pub const DEV_ADDR: u8 = 0x27;

#[test]
fn can_create_and_destroy() {
    let mut i2c = I2cMock::new(&[]);
    let dev = SparkfunQwiicGpio::new(&mut i2c);
    dev.destroy().done();
}

#[test]
fn can_init() {
    let expectations = [
        I2cTrans::write(DEV_ADDR, vec![0x03, 0xFF]), // Config register init
        I2cTrans::write(DEV_ADDR, vec![0x01, 0x00]), // OutputPort register init
        I2cTrans::write(DEV_ADDR, vec![0x02, 0x00]), // Polarity register init
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut dev = SparkfunQwiicGpio::new(&mut i2c);
    dev.init().unwrap();
    dev.destroy().done();
}
