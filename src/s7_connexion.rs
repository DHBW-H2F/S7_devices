use std::collections::HashMap;

use crate::{
    errors::S7Error,
    types::{Register, RegisterValue},
};

#[trait_variant::make(S7Connexion: Send)]
pub trait LocalS7Connexion {
    async fn connect(&mut self) -> Result<(), S7Error>;
    async fn read_register(&mut self, reg: Register) -> Result<RegisterValue, S7Error>;
    fn get_register_by_name(&self, name: String) -> Option<&Register>;
    async fn read_register_by_name(&mut self, name: String) -> Result<RegisterValue, S7Error>;
    async fn read_registers(
        &mut self,
        regs: Vec<Register>,
    ) -> Result<HashMap<String, RegisterValue>, S7Error>;
    async fn write_register(&mut self, reg: Register, val: RegisterValue) -> Result<(), S7Error>;
    async fn write_register_by_name(
        &mut self,
        name: String,
        val: RegisterValue,
    ) -> Result<(), S7Error>;
    async fn dump_registers(&mut self) -> Result<HashMap<String, RegisterValue>, S7Error>;
}
