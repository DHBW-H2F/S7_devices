use std::net::SocketAddr;

use s7_client::{Options, S7Client};

pub mod errors;
pub mod types;
pub mod utils;

use errors::S7Error;

pub struct S7Device {
    option: Options,
    client: Option<S7Client>,
}

impl S7Device {
    pub fn new(addr: SocketAddr) -> Self {
        let option = Options::new(
            addr.ip(),
            addr.port(),
            s7_client::ConnectMode::RackSlot {
                conn_type: s7_client::ConnectionType::PG,
                rack: 0,
                slot: 0,
            },
        );
        S7Device {
            option,
            client: None,
        }
    }
}

#[trait_variant::make(ModbusFactory: Send)]
pub trait S7Connexion {
    async fn connect(&mut self) -> Result<(), S7Error>;
}

impl S7Connexion for S7Device {
    async fn connect(&mut self) -> Result<(), S7Error> {
        self.client = Some(S7Client::connect(self.option.clone()).await?);
        Ok(())
    }
}
