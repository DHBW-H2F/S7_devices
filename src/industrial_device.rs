use std::collections::HashMap;

use crate::errors::S7Error;
use crate::s7_connexion::S7Connexion;
use crate::types::RegisterValue;
use crate::S7Device;

use industrial_device::errors::IndustrialDeviceError;
use industrial_device::types::Value;
use industrial_device::IndustrialDevice;

use async_trait::async_trait;

#[async_trait]
impl IndustrialDevice for S7Device {
    /// The `connect` function establishes a connection to an S7 device.
    /// 
    /// Returns:
    /// 
    /// The `connect` function is returning a `Result` enum with either `Ok(())` if the connection is
    /// successful or an `IndustrialDeviceError` if there is an error during the connection process.
    async fn connect(&mut self) -> Result<(), IndustrialDeviceError> {
        S7Connexion::connect(self).await?;
        Ok(())
    }

    /// The `dump_registers` retrieves register values from an S7
    /// connection and converts them into a HashMap of string keys and values.
    /// 
    /// Returns:
    /// 
    /// The `dump_registers` method is returning a `Result` containing a `HashMap<String, Value>` or an
    /// `IndustrialDeviceError`.
    async fn dump_registers(&mut self) -> Result<HashMap<String, Value>, IndustrialDeviceError> {
        let vals = S7Connexion::dump_registers(self).await?;
        Ok(vals
            .iter()
            .map(|(name, val)| (name.clone(), Into::<Value>::into(*val)))
            .collect())
    }

    /// The function `read_register_by_name` reads a register by name of an S7 device .
    /// 
    /// Arguments:
    /// 
    /// * `name`: name of the register that you want to read.
    /// 
    /// Returns:
    /// 
    /// The function `read_register_by_name` is returning a `Result` containing a `Value` or an
    /// `IndustrialDeviceError`.
    async fn read_register_by_name(&mut self, name: &str) -> Result<Value, IndustrialDeviceError> {
        Ok(S7Connexion::read_register_by_name(self, name).await?.into())
    }

    /// The function `write_register_by_name` writes a register value by name.
    /// 
    /// Arguments:
    /// 
    /// * `name`: The `name` parameter in the `write_register_by_name` function is a reference to a
    /// string that represents the name of the register you want to write to.
    /// * `value`: The `value` parameter in the `write_register_by_name` function is of type `&Value`,
    /// which is a reference to a value that needs to be written to a register.
    /// 
    /// Returns:
    /// 
    /// The `write_register_by_name` function is returning a `Result` enum with the possible outcomes
    /// being `Ok(())` if the operation is successful or an `Err(IndustrialDeviceError)` if an error
    /// occurs during the operation.
    async fn write_register_by_name(
        &mut self,
        name: &str,
        value: &Value,
    ) -> Result<(), IndustrialDeviceError> {
        let val: RegisterValue = value.clone().try_into()?;
        Ok(S7Connexion::write_register_by_name(self, name, &val).await?)
    }
}

impl From<S7Error> for IndustrialDeviceError {
    fn from(value: S7Error) -> Self {
        match value {
            S7Error::S7ClientError { err } => {
                IndustrialDeviceError::DeviceNotAccessibleError { err: Box::new(err) }
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

impl From<RegisterValue> for Value {
    fn from(value: RegisterValue) -> Self {
        match value {
            RegisterValue::S16(val) => Value::S16(val),
            RegisterValue::S32(val) => Value::S32(val),
            RegisterValue::Float32(val) => Value::Float32(val),
            RegisterValue::Boolean(val) => Value::Boolean(val),
        }
    }
}

impl TryFrom<Value> for RegisterValue {
    fn try_from(value: Value) -> Result<Self, IndustrialDeviceError> {
        let res = match value {
            Value::S16(val) => RegisterValue::S16(val),
            Value::S32(val) => RegisterValue::S32(val),
            Value::Float32(val) => RegisterValue::Float32(val),
            Value::Boolean(val) => RegisterValue::Boolean(val),
            _ => {
                return Err(IndustrialDeviceError::WrongValueType {
                    val: format!("{value:?}"),
                })
            }
        };
        Ok(res)
    }

    type Error = IndustrialDeviceError;
}
