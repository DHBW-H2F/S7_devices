use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DataType {
    BOOL,
    FLOAT,
}

pub enum RegAddress {
    Byte(ByteAddress),
    Bit(BitAddress),
}

#[derive(Deserialize)]
pub struct BitAddress {
    pub byte: ByteAddress,
    pub bit: u16,
}
#[derive(Deserialize)]
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

pub struct Register {
    pub data_type: DataType,
    pub name: String,
    pub addr: RegAddress,
}
