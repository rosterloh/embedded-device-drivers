use crate::{
    END_OF_PACKET, Error, READ_COMMAND, Register, SensorFrameMeasurement, SensorInfo, SingleTact,
    WRITE_COMMAND,
};
use embedded_hal::i2c::ErrorType;
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "SingleTact",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C> SingleTact<I2C>
where
    I2C: AsyncI2c + ErrorType,
{
    /// Set the device address.
    pub async fn set_address(&mut self, address: u8) -> Result<(), Error<I2C::Error>> {
        self.write_register(Register::ADDRESS, address).await?;
        self.address = address;
        Ok(())
    }

    /// Get sensor information.
    pub async fn get_info(&mut self) -> Result<SensorInfo, Error<I2C::Error>> {
        let address = self.read_register(Register::ADDRESS).await?;
        let mut data = [0; 2];
        self.read_registers(Register::SERIAL, &mut data).await?;
        let serial = u16::from(data[0]) << 8 | u16::from(data[1]);
        let firmware = self.read_register(Register::FIRMWARE).await?;
        self.read_registers(Register::BASELINE, &mut data).await?;
        let baseline = u16::from(data[0]) << 8 | u16::from(data[1]);
        Ok(SensorInfo {
            address,
            serial,
            firmware,
            baseline,
        })
    }

    /// Read the measurement data of all channels at once.
    pub async fn read_sensor_frame(&mut self) -> Result<SensorFrameMeasurement, Error<I2C::Error>> {
        let mut data = [0; 6];
        self.read_registers(Register::OUTPUT_DATA, &mut data)
            .await?;
        Ok(SensorFrameMeasurement {
            index: u16::from(data[0]) << 8 | u16::from(data[1]),
            timestamp: u16::from(data[2]) << 8 | u16::from(data[3]),
            output: u16::from(data[4]) << 8 | u16::from(data[5]),
        })
    }

    /// Write to a register.
    async fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<I2C::Error>> {
        let mut buffer = [WRITE_COMMAND, register, 0x01, value, END_OF_PACKET];
        self.i2c
            .write(self.address, &mut buffer)
            .await
            .map_err(Error::I2C)
    }

    /// Read a register.
    async fn read_register(&mut self, reg: u8) -> Result<u8, Error<I2C::Error>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(
                self.address,
                &[READ_COMMAND, reg, 0x01, END_OF_PACKET],
                &mut buffer,
            )
            .await
            .map_err(Error::I2C)
            .and(Ok(buffer[0]))
    }

    /// Read multiple registers.
    async fn read_registers(&mut self, reg: u8, data: &mut [u8]) -> Result<(), Error<I2C::Error>> {
        let length = data.len() as u8;
        self.i2c
            .write_read(
                self.address,
                &[READ_COMMAND, reg, length, END_OF_PACKET],
                data,
            )
            .await
            .map_err(Error::I2C)
    }
}
