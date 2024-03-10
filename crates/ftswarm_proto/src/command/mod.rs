use crate::command::direct::FtSwarmDirectCommand;
use crate::command::rpc::FtSwarmRPCCommand;
use crate::Serialized;

pub mod enums;
pub mod direct;
pub mod rpc;
pub mod argument;

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