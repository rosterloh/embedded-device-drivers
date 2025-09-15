use linux_embedded_hal::I2cdev;
use sparkfun_qwiic_gpio::SparkfunQwiicGpio;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = SparkfunQwiicGpio::new(dev);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sensor.init().unwrap();
    sensor.set_port_config(0x00).unwrap();
    for i in 0..8 {
        println!("Toggling port {}", i);
        sensor
            .set_pin_output(i, sparkfun_qwiic_gpio::PinLevel::High)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        sensor
            .set_pin_output(i, sparkfun_qwiic_gpio::PinLevel::Low)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
