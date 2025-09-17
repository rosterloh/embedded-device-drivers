use linux_embedded_hal::I2cdev;
use singletact::{DEFAULT_DEVICE_ADDRESS, SingleTact};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = SingleTact::new(i2c, DEFAULT_DEVICE_ADDRESS);
    let info = sensor.get_info().unwrap();
    println!("{:?}", info);
}
