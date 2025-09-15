use serde::{Deserialize, Serialize};

use crate::errors::S7Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataType {
    #[serde(alias = "BIT")]
    BOOL,
    FLOAT,
    INT32,
    INT16,
}

#[derive(Debug, Clone, Copy)]
/// The `RegisterValue` is defining different variants to represent values
/// that can be stored in a register.
pub enum RegisterValue {
    S16(i16),
    S32(i32),
    Float32(f32),
    Boolean(bool),
}

impl TryFrom<RegisterValue> for bool {
    type Error = S7Error;

    fn try_from(value: RegisterValue) -> Result<Self, Self::Error> {
        match value {
            RegisterValue::Boolean(val) => Ok(val),
            _ => Err(S7Error::InvalidRegisterValue),
        }
    }
}

impl TryFrom<RegisterValue> for i16 {
    type Error = S7Error;

    fn try_from(value: RegisterValue) -> Result<Self, Self::Error> {
        match value {
            RegisterValue::S16(val) => Ok(val),
            _ => Err(S7Error::InvalidRegisterValue),
        }
    }
}

impl TryFrom<RegisterValue> for Vec<u8> {
    type Error = S7Error;

    fn try_from(value: RegisterValue) -> Result<Self, Self::Error> {
        match value {
            RegisterValue::S16(val) => Ok(Vec::from(val.to_be_bytes())),
            RegisterValue::S32(val) => Ok(Vec::from(val.to_be_bytes())),
            RegisterValue::Float32(val) => Ok(Vec::from(val.to_be_bytes())),
            RegisterValue::Boolean(_) => Err(S7Error::InvalidRegisterValue),
        }
    }
}

#[derive(Debug, Clone)]
/// The `RegAddress` is defining different variants to represent a register a `Byte adress` or a `Bit Adress`
pub enum RegAddress {
    Byte(ByteAddress),
    Bit(BitAddress),
}


#[derive(Debug, Deserialize, Clone)]
/// Represents a memory address at the **bit** level in an S7 controller.
/// 
///
/// Properties:
/// * `db`: Number of the **Data Block** (DB) in which the address is located.
/// * `byte`: Index of the byte in the DB.
/// * `bit`: Index of the bit in the byte (value between `0` and `7`).
///
/// # Exemple
/// ```rust
/// let addr = BitAddress { db: 1, byte: 10, bit: 3 };
/// /*  Represents DB1.DBX10.3 in Siemens notation */
/// ```
pub struct BitAddress {
    pub db: u16,
    pub byte: u16,
    pub bit: u8,
}
#[derive(Debug, Deserialize, Clone)]

/// Represents a memory address at the **byte** level in an S7 PLC.
/// 
/// Unlike `BitAddress`, this structure is used when you want to access an entire register (1 byte or more),
///  without targeting a specific bit.
///
/// Properties:
/// * `db`: **Data Block** (DB) number.
/// * `byte`: Index of the byte within the DB.
/// 
/// # Exemple
/// ```rust
/// let addr = ByteAddress { db: 1, byte: 20 };
/// /* Represents DB1.DBB20 in Siemens notation */
/// ```
pub struct ByteAddress {
    pub db: u16,
    pub byte: u16,
}

impl From<ByteAddress> for RegAddress {
    fn from(value: ByteAddress) -> Self {
        RegAddress::Byte(value)
    }
}
impl From<BitAddress> for RegAddress {
    fn from(value: BitAddress) -> Self {
        RegAddress::Bit(value)
    }
}

impl TryFrom<RegAddress> for BitAddress {
    type Error = S7Error;

    fn try_from(value: RegAddress) -> Result<Self, Self::Error> {
        match value {
            RegAddress::Byte(_val) => Err(S7Error::MismatchedRegisterLengthError),
            RegAddress::Bit(val) => Ok(val),
        }
    }
}

impl TryFrom<RegAddress> for ByteAddress {
    type Error = S7Error;

    fn try_from(value: RegAddress) -> Result<Self, Self::Error> {
        match value {
            RegAddress::Byte(val) => Ok(val),
            RegAddress::Bit(_) => Err(S7Error::MismatchedRegisterLengthError),
        }
    }
}

#[derive(Debug, Clone)]
/// Format of a register
/// 
/// Parameters :
/// 
/// * `data_type` (`DataType`) - the type of the data of the register
/// * `name` (`String`) - the name of the register
/// * `addr` (`RegAddress`) - the adresse of the register
/// 
/// ```
pub struct Register {
    pub data_type: DataType,
    pub name: String,
    pub addr: RegAddress,
}

impl TryFrom<(Vec<u8>, Register)> for RegisterValue {
    type Error = S7Error;

    fn try_from((raw, datatype): (Vec<u8>, Register)) -> Result<Self, Self::Error> {
        match datatype.data_type {
            DataType::BOOL => {
                let byte = raw.get(0);
                if byte.is_none() {
                    return Err(S7Error::MismatchedRegisterLengthError);
                }
                let addr = match datatype.addr {
                    RegAddress::Byte(_) => return Err(S7Error::MismatchedRegisterLengthError),
                    RegAddress::Bit(addr) => addr,
                };
                let bit = byte.ok_or(S7Error::MismatchedRegisterLengthError)? & (1 << addr.bit);
                Ok(RegisterValue::Boolean(bit != 0))
            }
            DataType::FLOAT => {
                let val = f32::from_be_bytes(match raw.try_into() {
                    Ok(val) => val,
                    Err(_err) => return Err(S7Error::MismatchedRegisterLengthError),
                });
                Ok(RegisterValue::Float32(val))
            }
            DataType::INT32 => {
                let val = i32::from_be_bytes(match raw.try_into() {
                    Ok(val) => val,
                    Err(_err) => return Err(S7Error::MismatchedRegisterLengthError),
                });
                Ok(RegisterValue::S32(val))
            }
            DataType::INT16 => {
                let val = i16::from_be_bytes(match raw.try_into() {
                    Ok(val) => val,
                    Err(_err) => return Err(S7Error::MismatchedRegisterLengthError),
                });
                Ok(RegisterValue::S16(val))
            }
        }
    }
}
