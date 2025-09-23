use embedded_hal::delay::DelayNs;
// use embedded_hal_bus::i2c as i2c_bus;
// use embedded_hal_bus::util::AtomicCell;
use linux_embedded_hal::{Delay, I2cdev};
// use pca9548::prelude::*;
use singletact::{DEFAULT_DEVICE_ADDRESS, SingleTact};
// use sparkfun_qwiic_gpio::{ALL_OUTPUTS, PinLevel, SparkfunQwiicGpio};
// use std::cell::RefCell;

const MUX_BASE: u8 = 30;

fn main() {
    // let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    // let shared_i2c = RefCell::new(i2c);
    // let i2c_cell = AtomicCell::new(i2c);
    let mut delay = Delay;

    // let mut gpio = SparkfunQwiicGpio::new(&i2c_bus::RefCellDevice::new(&shared_i2c));
    // let mut gpio = SparkfunQwiicGpio::new(i2c_bus::AtomicDevice::new(&i2c_cell));
    // let mut mux = PCA9548::new(i2c_bus::AtomicDevice::new(&i2c_cell))
    //     .with_ports_disabled()
    //     .unwrap();
    // delay.delay_ms(100u32);
    // gpio.init().unwrap();
    // gpio.set_port_config(ALL_OUTPUTS).unwrap();

    for i in 0..8 {
        let bus = MUX_BASE + i;
        let i2c = I2cdev::new(format!("/dev/i2c-{}", bus)).unwrap();

        let id = 0x10 + i;
        println!(
            "Assigning sensor in position {} on bus {} address 0x{:02x}",
            i, bus, id
        );
        // gpio.set_pin_output(i, PinLevel::High).unwrap();
        // mux.set_port(i, true).unwrap();
        // delay.delay_ms(500u32);

        // let mut sensor = SingleTact::new(
        //     // &i2c_bus::RefCellDevice::new(&shared_i2c),
        //     i2c_bus::AtomicDevice::new(&i2c_cell),
        //     DEFAULT_DEVICE_ADDRESS,
        // );
        let mut sensor = SingleTact::new(i2c, DEFAULT_DEVICE_ADDRESS);
        let initial_info = sensor.get_info();
        if let Err(e) = initial_info {
            eprintln!("No response from sensor at default address: {:?}", e);
            continue;
        }
        // println!("{:?}", initial_info.unwrap());
        if let Err(e) = sensor.set_address(id) {
            eprintln!("No response from sensor at default address: {:?}", e);
            continue;
        }
        let final_info = sensor.get_info();
        match final_info {
            Ok(info) => {
                let addr = initial_info.unwrap().address;
                if addr != info.address {
                    println!("Address changed from {} to {}", addr, info.address);
                } else {
                    println!("Address changed to {} successfully.", info.address);
                }
            }
            Err(e) => eprintln!("Failed to get info from sensor at new address: {:?}", e),
        }

        // gpio.set_pin_output(i, PinLevel::Low).unwrap();
        // mux.set_port(i, false).unwrap();
        delay.delay_ms(1000u32);
    }
}
