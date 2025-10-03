use crate::{
    MAX_PORTS, PCA9548,
    error::{PCA9548Error, Result},
};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "PCA9548",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C> PCA9548<I2C>
where
    I2C: AsyncI2c,
{
    /// Disables all ports
    pub async fn with_ports_disabled(self) -> Result<Self, I2C::Error> {
        self.with_ports([false; MAX_PORTS]).await
    }

    /// Disables all ports
    pub async fn set_ports_disabled(mut self) -> Result<(), I2C::Error> {
        self.set_ports([false; MAX_PORTS]).await
    }

    /// Enables all ports
    pub async fn with_ports_enabled(self) -> Result<Self, I2C::Error> {
        self.with_ports([true; MAX_PORTS]).await
    }

    /// Enables all ports
    pub async fn set_ports_enabled(mut self) -> Result<(), I2C::Error> {
        self.set_ports([true; MAX_PORTS]).await
    }

    /// Enables / Disables the selected port
    pub async fn set_port(&mut self, port: u8, state: impl Into<bool>) -> Result<(), I2C::Error> {
        if port >= MAX_PORTS as u8 {
            return Err(PCA9548Error::PortError);
        }

        self.state[port as usize] = state.into();

        let code = Self::port_code(self.state);

        self.i2c_write(&[code]).await
    }

    /// Sets the selected port
    pub async fn with_port(mut self, port: u8, state: impl Into<bool>) -> Result<Self, I2C::Error> {
        self.set_port(port, state.into()).await?;
        Ok(self)
    }

    /// Enables / Disables the selected ports
    pub async fn set_ports(&mut self, ports: [bool; MAX_PORTS]) -> Result<(), I2C::Error> {
        let code = Self::port_code(ports);
        self.i2c_write(&[code]).await
    }

    /// Enables / Disables the selected ports
    pub async fn with_ports(mut self, ports: [bool; MAX_PORTS]) -> Result<Self, I2C::Error> {
        self.set_ports(ports).await?;
        Ok(self)
    }

    async fn i2c_write(&mut self, bytes: &[u8]) -> Result<(), I2C::Error> {
        self.i2c
            .write(self.address, bytes)
            .await
            .map_err(PCA9548Error::I2CError)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_hal_mock::common::Generic;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use rstest::*;

    impl PCA9548<Generic<Transaction>> {
        fn done(mut self) {
            self.i2c.done();
        }
    }

    #[rstest]
    #[case([true;8], 0b0000_1111)]
    #[case([false;8], 0b0000_0000)]
    #[case([true, false, true, false, false, true, false, true], 0b1010_0101)]
    fn setup_ports(#[case] ports: [bool; 8], #[case] result: u8) {
        assert_eq!(PCA9548::<Mock>::port_code(ports), result)
    }

    #[rstest]
    #[case([true;3], 0b1110_0111)]
    #[case([false;3], 0b1110_0000)]
    #[case([true, false, false], 0b1110_0001)]
    #[case([false, true, false], 0b1110_0010)]
    #[case([true, false, true], 0b1110_0101)]
    fn setup_address(#[case] addr: [bool; 3], #[case] result: u8) {
        let i2c = Mock::new(&[]);
        let multiplexer = PCA9548::new(i2c).with_address_pins(addr[0], addr[1], addr[2]);
        assert_eq!(multiplexer.address, result);
        multiplexer.done();
    }
}
