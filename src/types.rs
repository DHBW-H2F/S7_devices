use serde::{Deserialize, Serialize};

use crate::errors::{MismatchedRegisterLengthError, S7Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataType {
    BOOL,
    FLOAT,
    INT32,
    INT16,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterValue {
    S16(i16),
    S32(i32),
    Float32(f32),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum RegAddress {
    Byte(ByteAddress),
    Bit(BitAddress),
}

#[derive(Debug, Deserialize, Clone)]
pub struct BitAddress {
    pub db: u16,
    pub byte: u16,
    pub bit: u16,
}
#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Clone)]
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
                    return Err(MismatchedRegisterLengthError.into());
                }
                let addr = match datatype.addr {
                    RegAddress::Byte(addr) => return Err(MismatchedRegisterLengthError.into()),
                    RegAddress::Bit(addr) => addr,
                };
                let bit = byte.unwrap() & (1 << addr.bit);
                Ok(RegisterValue::Boolean(bit != 0))
            }
            DataType::FLOAT => todo!(),
            DataType::INT32 => todo!(),
            DataType::INT16 => todo!(),
        }
    }
}
