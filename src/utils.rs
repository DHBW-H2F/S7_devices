use std::{collections::HashMap, fs::File};

use de_regex;
use serde::{Deserialize, Serialize};

use crate::types::{BitAddress, ByteAddress, DataType, RegAddress, Register};

#[derive(Serialize, Deserialize)]
struct RegistersFormat {
    pub name: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_: DataType,
}

const BYTE_ADDRESS_REGEX: &str = r"^DB(?P<db>\d+)\.DBD(?P<byte>\d+)$";
const WORD_ADDRESS_REGEX: &str = r"^DB(?P<db>\d+)\.DBW(?P<byte>\d+)$";
const BIT_ADDRESS_REGEX: &str = r"^DB(?P<db>\d+)\.DBX(?P<byte>\d+)\.(?P<bit>\d+)$";

#[derive(Debug)]
pub enum JsonReadError {
    SerdeJson(serde_json::Error),
    DeRegex(de_regex::Error),
}

impl From<de_regex::Error> for JsonReadError {
    fn from(value: de_regex::Error) -> Self {
        JsonReadError::DeRegex(value)
    }
}
impl From<serde_json::Error> for JsonReadError {
    fn from(value: serde_json::Error) -> Self {
        JsonReadError::SerdeJson(value)
    }
}

pub fn get_defs_from_json(input: File) -> Result<HashMap<String, Register>, JsonReadError> {
    let raw: Vec<RegistersFormat> = serde_json::from_reader(input)?;
    let mut m = HashMap::<String, Register>::new();
    for f in raw {
        let addr: RegAddress = match f.type_ {
            DataType::BOOL => {
                let res: BitAddress = de_regex::from_str(f.id.as_str(), BIT_ADDRESS_REGEX)?;
                res.into()
            }
            DataType::FLOAT | DataType::INT32 => {
                let res: ByteAddress = de_regex::from_str(f.id.as_str(), BYTE_ADDRESS_REGEX)?;
                res.into()
            }
            DataType::INT16 => {
                let res: ByteAddress = de_regex::from_str(f.id.as_str(), WORD_ADDRESS_REGEX)?;
                res.into()
            }
        };
        m.insert(
            f.name.clone(),
            Register {
                name: f.name,
                addr,
                data_type: f.type_,
            },
        );
    }
    Ok(m)
}
