use std::{collections::HashMap, net::SocketAddr};

use s7_client::{s7_comm::DataItemVal, Area, BitAddr, Options, S7Client};

pub mod errors;
pub mod types;
pub mod utils;

use errors::{DeviceNotConnectedError, RegisterDoesNotExistsError, S7Error};
use types::{RegAddress, Register, RegisterValue};

pub struct S7Device {
    option: Options,
    client: Option<S7Client>,
    registers: HashMap<String, Register>,
}

impl S7Device {
    pub fn new(addr: SocketAddr, regs: HashMap<String, Register>) -> Self {
        let option = Options::new(
            addr.ip(),
            addr.port(),
            s7_client::ConnectMode::RackSlot {
                conn_type: s7_client::ConnectionType::PG,
                rack: 0,
                slot: 1,
            },
        );
        S7Device {
            option,
            client: None,
            registers: regs,
        }
    }
}

#[trait_variant::make(ModbusFactory: Send)]
pub trait S7Connexion {
    async fn connect(&mut self) -> Result<(), S7Error>;
    async fn read_register(&mut self, reg: Register) -> Result<RegisterValue, S7Error>;
    fn get_register_by_name(&self, name: String) -> Option<&Register>;
    async fn read_register_by_name(&mut self, name: String) -> Result<RegisterValue, S7Error>;
    async fn read_registers(
        &mut self,
        regs: Vec<Register>,
    ) -> Result<HashMap<String, RegisterValue>, S7Error>;
    async fn dump_registers(&mut self) -> Result<HashMap<String, RegisterValue>, S7Error>;
}

impl S7Connexion for S7Device {
    async fn connect(&mut self) -> Result<(), S7Error> {
        self.client = Some(S7Client::connect(self.option.clone()).await?);
        Ok(())
    }

    async fn read_register(&mut self, reg: Register) -> Result<RegisterValue, S7Error> {
        if self.client.is_none() {
            return Err(DeviceNotConnectedError.into());
        }
        let area = match reg.data_type {
            types::DataType::BOOL => match reg.addr.clone() {
                RegAddress::Byte(val) => panic!("Mismatched register type and address ({val:?})"),
                RegAddress::Bit(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Byte {
                        addr: addr.byte,
                        len: 1,
                    },
                ),
            },
            types::DataType::FLOAT => match reg.addr.clone() {
                RegAddress::Byte(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Real {
                        addr: addr.byte,
                        len: 4,
                    },
                ),
                RegAddress::Bit(_) => todo!(),
            },
            types::DataType::INT32 => match reg.addr.clone() {
                RegAddress::Byte(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Int {
                        addr: addr.byte,
                        len: 4,
                    },
                ),
                RegAddress::Bit(_) => todo!(),
            },
            types::DataType::INT16 => match reg.addr.clone() {
                RegAddress::Byte(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Int {
                        addr: addr.byte,
                        len: 2,
                    },
                ),
                RegAddress::Bit(_) => todo!(),
            },
        };

        let rec_val = self.client.as_mut().unwrap().read(vec![area]).await?;
        println!("{rec_val:?}");
        let raw: Option<&DataItemVal> = rec_val.get(0);

        let bytes: Vec<u8> = raw.unwrap().data.clone();
        let conv: RegisterValue = (bytes, reg).try_into()?;
        Ok(conv)
    }

    async fn read_register_by_name(&mut self, name: String) -> Result<RegisterValue, S7Error> {
        let reg = self.get_register_by_name(name);

        match reg {
            Some(reg) => self.read_register(reg.clone()).await,
            None => return Err(RegisterDoesNotExistsError.into()),
        }
    }

    fn get_register_by_name(&self, name: String) -> Option<&Register> {
        self.registers.get(&name)
    }

    async fn dump_registers(&mut self) -> Result<HashMap<String, RegisterValue>, S7Error> {
        self.read_registers(self.registers.clone().into_values().collect())
            .await
    }

    async fn read_registers(
        &mut self,
        regs: Vec<Register>,
    ) -> Result<HashMap<String, RegisterValue>, S7Error> {
        let mut res: HashMap<String, RegisterValue> = HashMap::with_capacity(regs.len());
        for reg in regs {
            let val = self.read_register(reg.clone()).await?;
            res.insert(reg.name.clone(), val);
        }
        Ok(res)
    }
}
