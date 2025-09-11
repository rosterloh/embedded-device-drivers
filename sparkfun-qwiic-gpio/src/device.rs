use crate::{Error, PinConfig, PinLevel, PinPolarity, Register, SparkfunQwiicGpio};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "SparkfunQwiicGpio",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> SparkfunQwiicGpio<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Initialise the device with default settings.
    pub async fn init(&mut self) -> Result<(), Error<E>> {
        // Set all pins as inputs (default state)
        self.write_register(Register::CONFIG, 0xFF).await?;

        // Set all outputs to low (when configured as outputs)
        self.write_register(Register::OUTPUT, 0x00).await?;

        // Set all polarities to normal (non-inverted)
        self.write_register(Register::POLARITY, 0x00).await?;

        Ok(())
    }

    /// Read all input pins at once.
    pub async fn read_input_port(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::INPUT).await
    }

    /// Read a specific input pin.
    pub async fn read_pin_input(&mut self, pin: u8) -> Result<PinLevel, Error<E>> {
        let port_value = self.read_input_port().await?;
        let pin_value = (port_value >> pin) & 0x01;
        Ok(if pin_value == 0 {
            PinLevel::Low
        } else {
            PinLevel::High
        })
    }

    /// Write all output pins at once.
    pub async fn write_output_port(&mut self, value: u8) -> Result<(), Error<E>> {
        self.write_register(Register::OUTPUT, value).await
    }

    /// Read current output port register value.
    pub async fn read_output_port(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::OUTPUT).await
    }

    /// Set a specific output pin.
    pub async fn set_pin_output(&mut self, pin: u8, level: PinLevel) -> Result<(), Error<E>> {
        let mut current_value = self.read_output_port().await?;
        match level {
            PinLevel::High => current_value |= 1 << pin,
            PinLevel::Low => current_value &= !(1 << pin),
        }
        self.write_output_port(current_value).await
    }

    /// Toggle a specific output pin.
    pub async fn toggle_pin_output(&mut self, pin: u8) -> Result<(), Error<E>> {
        let mut current_value = self.read_output_port().await?;
        current_value ^= 1 << pin;
        self.write_output_port(current_value).await
    }

    /// Configure pin direction (input/output).
    pub async fn set_pin_config(&mut self, pin: u8, config: PinConfig) -> Result<(), Error<E>> {
        let mut current_config = self.read_register(Register::CONFIG).await?;
        match config {
            PinConfig::Input => current_config |= 1 << pin,
            PinConfig::Output => current_config &= !(1 << pin),
        }
        self.write_register(Register::CONFIG, current_config).await
    }

    /// Configure all pins direction at once.
    pub async fn set_port_config(&mut self, config: u8) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, config).await
    }

    /// Set pin polarity (normal/inverted).
    pub async fn set_pin_polarity(
        &mut self,
        pin: u8,
        polarity: PinPolarity,
    ) -> Result<(), Error<E>> {
        let mut current_polarity = self.read_register(Register::POLARITY).await?;
        match polarity {
            PinPolarity::Normal => current_polarity &= !(1 << pin),
            PinPolarity::Inverted => current_polarity |= 1 << pin,
        }
        self.write_register(Register::POLARITY, current_polarity)
            .await
    }

    /// Configure all pins polarity at once.
    pub async fn set_port_polarity(&mut self, polarity: u8) -> Result<(), Error<E>> {
        self.write_register(Register::POLARITY, polarity).await
    }

    /// Read port configuration.
    pub async fn read_port_config(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::CONFIG).await
    }

    /// Read port polarity configuration.
    pub async fn read_port_polarity(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::POLARITY).await
    }

    /// Write to a register.
    async fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, value])
            .await
            .map_err(Error::I2C)
    }

    /// Read a register.
    async fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.address, &[reg], &mut buffer)
            .await
            .map_err(Error::I2C)
            .and(Ok(buffer[0]))
    }
}
