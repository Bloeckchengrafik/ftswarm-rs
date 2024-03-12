use crate::{NameOf, Serialized};

pub enum FtSwarmDirectCommand {
    Help,
    Setup,
    Halt,
    Whoami,
    Uptime,
    StartCli
}

impl NameOf for FtSwarmDirectCommand {
    fn name(&self) -> String {
        match self {
            FtSwarmDirectCommand::Help => "help".to_string(),
            FtSwarmDirectCommand::Setup => "setup".to_string(),
            FtSwarmDirectCommand::Halt => "halt".to_string(),
            FtSwarmDirectCommand::Whoami => "whoami".to_string(),
            FtSwarmDirectCommand::Uptime => "uptime".to_string(),
            FtSwarmDirectCommand::StartCli => "startCLI".to_string(),
        }
    }
}

impl Serialized for FtSwarmDirectCommand {
    fn serialize(&self) -> String {
        self.name()
    }
}