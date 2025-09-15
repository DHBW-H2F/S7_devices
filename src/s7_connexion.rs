use std::collections::HashMap;

use crate::{
    errors::S7Error,
    types::{Register, RegisterValue},
};

/// This code snippet defines a "interfaces" named `S7Connexion` reffering to the s7 device
pub trait S7Connexion {
    fn connect(&mut self) -> impl std::future::Future<Output = Result<(), S7Error>> + Send;
    fn read_register(
        &mut self,
        reg: &Register,
    ) -> impl std::future::Future<Output = Result<RegisterValue, S7Error>> + Send;
    fn get_register_by_name(&self, name: &str) -> Option<&Register>;
    fn read_register_by_name(
        &mut self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<RegisterValue, S7Error>> + Send;
    fn read_registers(
        &mut self,
        regs: &[Register],
    ) -> impl std::future::Future<Output = Result<HashMap<String, RegisterValue>, S7Error>> + Send;
    fn write_register(
        &mut self,
        reg: &Register,
        val: &RegisterValue,
    ) -> impl std::future::Future<Output = Result<(), S7Error>> + Send;
    fn write_register_by_name(
        &mut self,
        name: &str,
        val: &RegisterValue,
    ) -> impl std::future::Future<Output = Result<(), S7Error>> + Send;
    fn dump_registers(
        &mut self,
    ) -> impl std::future::Future<Output = Result<HashMap<String, RegisterValue>, S7Error>> + Send;
}
