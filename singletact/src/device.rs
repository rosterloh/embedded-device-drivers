use crate::{
    END_OF_PACKET, Error, READ_COMMAND, Register, SensorFrameMeasurement, SingleTact, WRITE_COMMAND,
};
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
impl<I2C, E> SingleTact<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Set the device address.
    pub async fn set_address(&mut self, address: u8) -> Result<(), Error<E>> {
        self.write_register(Register::ADDRESS, address).await?;
        self.address = address;
        Ok(())
    }

    /// Read the measurement data of all channels at once.
    pub async fn read_sensor_frame(&mut self) -> Result<SensorFrameMeasurement, Error<E>> {
        let mut data = [0; 6];
        self.read_registers(Register::OUTPUT_DATA, &mut data)
            .await?;
        Ok(SensorFrameMeasurement {
            index: u16::from(data[1]) << 8 | u16::from(data[0]),
            timestamp: u16::from(data[3]) << 8 | u16::from(data[2]),
            output: u16::from(data[5]) << 8 | u16::from(data[4]),
        })
    }

    /// Write to a register.
    async fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        let mut buffer = [WRITE_COMMAND, register, 0x01, value, END_OF_PACKET];
        self.i2c
            .write(self.address, &mut buffer)
            .await
            .map_err(Error::I2C)
    }

    /// Read a register.
    // async fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
    //     let mut buffer = [0u8; 1];
    //     self.i2c
    //         .write_read(
    //             self.address,
    //             &[READ_COMMAND, reg, 0x01, END_OF_PACKET],
    //             &mut buffer,
    //         )
    //         .await
    //         .map_err(Error::I2C)
    //         .and(Ok(buffer[0]))
    // }

    /// Read multiple registers.
    async fn read_registers(&mut self, reg: u8, data: &mut [u8]) -> Result<(), Error<E>> {
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
