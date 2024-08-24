use std::collections::HashMap;

use crate::errors::S7Error;
use crate::s7_connexion::S7Connexion;
use crate::S7Device;

use industrial_device::errors::IndustrialDeviceError;
use industrial_device::types::Value;
use industrial_device::IndustrialDevice;

impl IndustrialDevice for S7Device {
    async fn connect(&mut self) -> Result<(), IndustrialDeviceError> {
        S7Connexion::connect(self).await?;
        Ok(())
    }

    async fn dump_registers(&mut self) -> Result<HashMap<String, Value>, IndustrialDeviceError> {
        todo!()
    }
}

impl From<S7Error> for IndustrialDeviceError {
    fn from(value: S7Error) -> Self {
        match value {
            S7Error::S7ClientError { err } => {
                IndustrialDeviceError::RequestError { err: Box::new(err) }
            }
            S7Error::DeviceNotConnectedError => IndustrialDeviceError::DeviceNotConnectedError {
                err: Box::new(value),
            },
            S7Error::MismatchedRegisterLengthError => IndustrialDeviceError::RequestError {
                err: Box::new(value),
            },
            S7Error::RegisterDoesNotExistsError => IndustrialDeviceError::RequestError {
                err: Box::new(value),
            },
            S7Error::InvalidRegisterValue => IndustrialDeviceError::RequestError {
                err: Box::new(value),
            },
        }
    }
}
