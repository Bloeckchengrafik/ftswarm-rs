use crate::{Deserialized, NameOf, Serialized};

#[derive(Debug)]
pub enum FtSwarmDirectCommand {
    Help,
    Setup,
    Halt,
    Whoami,
    Uptime,
    StartCli,
    Custom(String),
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
            FtSwarmDirectCommand::Custom(name) => name.clone(),
        }
    }
}

impl Serialized for FtSwarmDirectCommand {
    fn serialize(&self) -> String {
        self.name()
    }
}

impl Deserialized for FtSwarmDirectCommand {
    fn deserialize(value: &String) -> Result<Self, String> where Self: Sized {
        match value.as_str() {
            "help" => Ok(FtSwarmDirectCommand::Help),
            "setup" => Ok(FtSwarmDirectCommand::Setup),
            "halt" => Ok(FtSwarmDirectCommand::Halt),
            "whoami" => Ok(FtSwarmDirectCommand::Whoami),
            "uptime" => Ok(FtSwarmDirectCommand::Uptime),
            "startCLI" => Ok(FtSwarmDirectCommand::StartCli),
            _ => Err(format!("Unknown command: {}", value))
        }
    }
}