use crate::command::direct::FtSwarmDirectCommand;
use crate::command::rpc::FtSwarmRPCCommand;
use crate::{Deserialized, Serialized};

pub mod enums;
pub mod direct;
pub mod rpc;
pub mod argument;

#[derive(Debug)]
pub enum FtSwarmCommand {
    RPC(FtSwarmRPCCommand),
    Direct(FtSwarmDirectCommand),
}

impl Serialized for FtSwarmCommand {
    fn serialize(&self) -> String {
        match self {
            FtSwarmCommand::RPC(cmd) => cmd.serialize(),
            FtSwarmCommand::Direct(cmd) => cmd.serialize(),
        }
    }
}

impl Deserialized for FtSwarmCommand {
    fn deserialize(s: &String) -> Result<FtSwarmCommand, String> {
        if s.contains(".") {
            Ok(FtSwarmCommand::RPC(FtSwarmRPCCommand::deserialize(s)?))
        } else {
            Ok(FtSwarmCommand::Direct(FtSwarmDirectCommand::deserialize(s)?))
        }
    }
}