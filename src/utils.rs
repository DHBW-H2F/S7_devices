use std::{collections::HashMap, fs::File};

use log::warn;

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
const BIT_ADDRESS_REGEX: &str = r"^DB(?P<db>\d+)\.DBX(?P<byte>\d+)\.(?P<bit>\d+)$";

pub fn get_defs_from_json(input: File) -> Result<HashMap<String, Register>, serde_json::Error> {
    let raw: Vec<RegistersFormat> = serde_json::from_reader(input)?;
    let mut m = HashMap::<String, Register>::new();
    for f in raw {
        let addr_res: Result<RegAddress, _> = match f.type_ {
            DataType::BOOL => {
                let res: Result<BitAddress, _> =
                    de_regex::from_str(f.id.as_str(), BIT_ADDRESS_REGEX);
                res.map(|v| v.into())
            }
            DataType::FLOAT => {
                let res: Result<ByteAddress, _> =
                    de_regex::from_str(f.id.as_str(), BYTE_ADDRESS_REGEX);
                res.map(|v| v.into())
            }
        };
        let addr: RegAddress = match addr_res {
            Ok(res) => res.into(),
            Err(err) => {
                warn!(
                    "Could not parse reg {0} ({1}) dropping it ({err})",
                    f.name, f.id
                );
                continue;
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
