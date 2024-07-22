use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    BOOL,
    FLOAT,
    INT32,
    INT16,
}

#[derive(Debug)]
pub enum RegAddress {
    Byte(ByteAddress),
    Bit(BitAddress),
}

#[derive(Debug, Deserialize)]
pub struct BitAddress {
    pub db: u16,
    pub byte: u16,
    pub bit: u16,
}
#[derive(Debug, Deserialize)]
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

#[derive(Debug)]
pub struct Register {
    pub data_type: DataType,
    pub name: String,
    pub addr: RegAddress,
}
