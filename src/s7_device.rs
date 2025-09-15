use std::{collections::HashMap, net::SocketAddr};

use s7_client::{s7_comm::DataItemVal, Area, Options, S7Client};

pub mod errors;
pub mod industrial_device;
pub mod s7_connexion;
pub mod types;
pub mod utils;

use errors::S7Error;
use s7_connexion::S7Connexion;
use types::{BitAddress, ByteAddress, RegAddress, Register, RegisterValue};

/// The `S7Device` struct represents a device with options, a client, and registers in Rust.
/// 
/// Properties:
/// 
/// * `option`: The `option` is used to store the configuration related to the device (IP, port, ConnectMode ).
/// * `client`: It allows the `S7Device` struct to have a client associated with it, but it can also be `None` if no
/// client is currently connected.
/// * `registers`: The `registers` property in the `S7Device` struct is a HashMap that stores key-value
/// pairs where the key is a `String` and the value is a `Register` struct. This allows you to store and
/// access registers by their unique names within the device.
pub struct S7Device {
    option: Options,
    client: Option<S7Client>,
    registers: HashMap<String, Register>,
}

impl S7Device {
    /// The function `new` creates a new `S7Device` instance with a specified address and register map.
    /// 
    /// Parameters :
    /// 
    /// * `addr`: a SocketAddr that represents a socket address, which includes an IP address
    /// and a port number.
    /// * `regs`: The `regs` parameter is a `HashMap<String, Register>`, which is the list of the register of the S7 device
    /// 
    /// Returns:
    /// 
    /// A new instance of the `S7Device` struct is being returned with the provided `SocketAddr` and
    /// `HashMap<String, Register>` as parameters.
    pub fn new(addr: SocketAddr, regs: HashMap<String, Register>) -> Self {
        let option = Options::new(
            addr.ip(),
            addr.port(),
            s7_client::ConnectMode::init_rack_slot(s7_client::ConnectionType::Basic, 0, 0),
        );
        S7Device {
            option,
            client: None,
            registers: regs,
        }
    }
}

impl S7Connexion for S7Device {
    /// Establishes a connection with the S7 device.
    ///
    /// Errors :
    /// Returns `S7Error` if the connection fails.
    async fn connect(&mut self) -> Result<(), S7Error> {
        self.client = Some(S7Client::connect(self.option.clone()).await?);
        Ok(())
    }


    /// Reads the value of a specific register from the S7 PLC.
    ///
    ///
    /// Parameters :
    /// - `reg`: description of the register to read (`Register`).
    ///
    /// Returns :
    /// The register value as a `RegisterValue`.
    ///
    /// Errors :
    /// - `DeviceNotConnectedError` if the client is not connected.
    /// - `MismatchedRegisterLengthError` if the address does not match
    ///   the expected type (e.g. `BOOL` on a `ByteAddress`).
    async fn read_register(&mut self, reg: &Register) -> Result<RegisterValue, S7Error> {
        self.client
            .as_ref()
            .ok_or(S7Error::DeviceNotConnectedError)?;

        let area = match reg.data_type {
            types::DataType::BOOL => match &reg.addr {
                RegAddress::Byte(_val) => return Err(S7Error::MismatchedRegisterLengthError),
                RegAddress::Bit(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Byte {
                        addr: addr.byte,
                        len: 1,
                    },
                ),
            },
            types::DataType::FLOAT => match &reg.addr {
                RegAddress::Byte(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Real {
                        addr: addr.byte,
                        len: 4,
                    },
                ),
                RegAddress::Bit(_) => return Err(S7Error::MismatchedRegisterLengthError),
            },
            types::DataType::INT32 => match &reg.addr {
                RegAddress::Byte(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Int {
                        addr: addr.byte,
                        len: 4,
                    },
                ),
                RegAddress::Bit(_) => return Err(S7Error::MismatchedRegisterLengthError),
            },
            types::DataType::INT16 => match &reg.addr {
                RegAddress::Byte(addr) => Area::DataBausteine(
                    addr.db,
                    s7_client::DataSizeType::Int {
                        addr: addr.byte,
                        len: 2,
                    },
                ),
                RegAddress::Bit(_) => return Err(S7Error::MismatchedRegisterLengthError),
            },
        };

        let rec_val = self.client.as_mut().unwrap().read(vec![area]).await?;
        let raw: Option<&DataItemVal> = rec_val.get(0);

        let bytes: Vec<u8> = raw.unwrap().data.clone();
        let conv: RegisterValue = (bytes, reg.clone()).try_into()?;
        Ok(conv)
    }

    /// Reads a register by its logical name defined in the configuration.
    ///
    /// Parameters :
    /// - `name`: the name of the register.
    ///
    /// Returns :
    /// The register value (`RegisterValue`).
    ///
    /// Errors :
    /// - `RegisterDoesNotExistsError` if no register with this name is defined.
    async fn read_register_by_name(&mut self, name: &str) -> Result<RegisterValue, S7Error> {
        let reg = self.get_register_by_name(name).cloned();

        match reg {
            Some(reg) => self.read_register(&reg).await,
            None => return Err(S7Error::RegisterDoesNotExistsError),
        }
    }

    /// Retrieves a register definition by its logical name.
    ///
    /// Parameters :
    /// - `name`: symbolic name of the register.
    ///
    /// Returns :
    /// A reference to the `Register` if found, otherwise `None`.
    fn get_register_by_name(&self, name: &str) -> Option<&Register> {
        self.registers.get(name)
    }

    /// Reads the value of **all known registers** of the device.
    ///
    /// Returns :
    /// - `HashMap<String, RegisterValue>`: table of register names and values.
    ///
    /// Errors :
    /// Propagates errors from `read_registers`.
    async fn dump_registers(&mut self) -> Result<HashMap<String, RegisterValue>, S7Error> {
        let regs: Vec<Register> = self.registers.values().map(|v| v.clone()).collect();
        self.read_registers(&regs).await
    }

    /// Reads multiple registers at once.
    ///
    /// Parameters :
    /// - `regs`: list of registers to read.
    ///
    /// Returns :
    /// - `HashMap<String, RegisterValue>`: mapping of register name → value.
    ///
    /// Errors :
    /// Propagates errors from `read_register`.
    async fn read_registers(
        &mut self,
        regs: &[Register],
    ) -> Result<HashMap<String, RegisterValue>, S7Error> {
        let mut res: HashMap<String, RegisterValue> = HashMap::with_capacity(regs.len());
        for reg in regs {
            let val = self.read_register(reg).await?;
            res.insert(reg.name.clone(), val);
        }
        Ok(res)
    }

    /// Writes a value to a specific register of the S7 PLC.
    ///
    /// Depending on the data type, the write is performed as:
    /// - `BOOL` → single bit write (`BitAddress`).
    /// - `FLOAT`, `INT16`, `INT32` → byte/block write (`ByteAddress`).
    ///
    /// Parameters :
    /// - `reg`: description of the register.
    /// - `val`: value to be written (`RegisterValue`).
    ///
    /// Errors :
    /// - `DeviceNotConnectedError` if the client is not connected.
    /// - `MismatchedRegisterLengthError` if the address does not match
    ///   the expected type.
    async fn write_register(&mut self, reg: &Register, val: &RegisterValue) -> Result<(), S7Error> {
        if self.client.is_none() {
            return Err(S7Error::DeviceNotConnectedError);
        }

        match reg.data_type {
            types::DataType::BOOL => {
                let addr: BitAddress = reg.addr.clone().try_into()?;
                self.client
                    .as_mut()
                    .unwrap()
                    .write_db_bit(addr.db, addr.byte, addr.bit, val.clone().try_into()?)
                    .await?
            }
            types::DataType::FLOAT | types::DataType::INT32 | types::DataType::INT16 => {
                let addr: ByteAddress = reg.addr.clone().try_into()?;
                let value: Vec<u8> = val.clone().try_into()?;
                self.client
                    .as_mut()
                    .unwrap()
                    .write_db_bytes(addr.db, addr.byte, &value)
                    .await?
            }
        };
        Ok(())
    }

    /// Writes a value to a register identified by its logical name.
    ///
    /// Parameters :
    /// - `name`: symbolic name of the register.
    /// - `val`: value to be written.
    ///
    /// Errors :
    /// - `RegisterDoesNotExistsError` if the register does not exist.
    async fn write_register_by_name(
        &mut self,
        name: &str,
        val: &RegisterValue,
    ) -> Result<(), S7Error> {
        let reg = self.get_register_by_name(name).cloned();

        match reg {
            Some(reg) => self.write_register(&reg, val).await,
            None => return Err(S7Error::RegisterDoesNotExistsError),
        }
    }
}