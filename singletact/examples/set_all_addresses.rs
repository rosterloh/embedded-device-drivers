use embedded_hal::delay::DelayNs;
use embedded_hal_bus::i2c as i2c_bus;
use embedded_hal_bus::util::AtomicCell;
use linux_embedded_hal::{Delay, I2cdev};
use singletact::{DEFAULT_DEVICE_ADDRESS, SingleTact};
use sparkfun_qwiic_gpio::{ALL_OUTPUTS, PinLevel, SparkfunQwiicGpio};
// use std::cell::RefCell;

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    // let shared_i2c = RefCell::new(i2c);
    let i2c_cell = AtomicCell::new(i2c);
    let mut delay = Delay;

    // let mut gpio = SparkfunQwiicGpio::new(&i2c_bus::RefCellDevice::new(&shared_i2c));
    let mut gpio = SparkfunQwiicGpio::new(i2c_bus::AtomicDevice::new(&i2c_cell));
    delay.delay_ms(100u32);
    gpio.init().unwrap();
    gpio.set_port_config(ALL_OUTPUTS).unwrap();

    for i in 0..4 {
        let id = 0x10 + i;
        println!("Configuring sensor at 0x{:02x}", id);
        gpio.set_pin_output(i, PinLevel::High).unwrap();
        delay.delay_ms(500u32);

        let mut sensor = SingleTact::new(
            // &i2c_bus::RefCellDevice::new(&shared_i2c),
            i2c_bus::AtomicDevice::new(&i2c_cell),
            DEFAULT_DEVICE_ADDRESS,
        );
        let info = sensor.get_info();
        if let Err(e) = info {
            println!("No response from sensor at default address: {:?}", e);
            continue;
        }
        println!("{:?}", info.unwrap());
        if let Err(e) = sensor.set_address(id) {
            println!("No response from sensor at default address: {:?}", e);
            continue;
        }

        gpio.set_pin_output(i, PinLevel::Low).unwrap();
        delay.delay_ms(1000u32);
    }
}
