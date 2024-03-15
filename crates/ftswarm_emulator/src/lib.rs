use std::collections::VecDeque;
use log::info;
use ftswarm_proto::command::direct::FtSwarmDirectCommand;
use ftswarm_proto::command::FtSwarmCommand;
use ftswarm_proto::command::rpc::{FtSwarmRPCCommand, RpcFunction};
use ftswarm_proto::Deserialized;
use ftswarm_serial::SwarmSerialPort;

pub struct EmulatedSerialPort(VecDeque<String>);

impl EmulatedSerialPort {
    pub fn new() -> EmulatedSerialPort {
        EmulatedSerialPort(VecDeque::new())
    }

    fn handle_direct_command(&mut self, command: FtSwarmDirectCommand) {
        match command {
            FtSwarmDirectCommand::Help => { self.0.push_back("Help".to_string()); }
            FtSwarmDirectCommand::Setup => { self.0.push_back("Setup".to_string()); }
            FtSwarmDirectCommand::Halt => {}
            FtSwarmDirectCommand::Whoami => {self.0.push_back("ftSwarm100/kelda".to_string()); }
            FtSwarmDirectCommand::Uptime => { self.0.push_back("uptime: 31.000 s".to_string()); }
            FtSwarmDirectCommand::StartCli => { self.0.push_back("@@@ ftSwarmOS CLI started".to_string()); }
        }
    }

    fn handle_rpc_command(&mut self, command: FtSwarmRPCCommand) {
        let functions_to_ok = vec![
            RpcFunction::Show,
            RpcFunction::TriggerUserEvent,
            RpcFunction::SetMicroStepMode,
            RpcFunction::SetSensorType,
            RpcFunction::OnTrigger,
            RpcFunction::SetActorType,
            RpcFunction::SetSpeed,
            RpcFunction::SetMotionType,
            RpcFunction::SetPosition,
            RpcFunction::SetOffset,
            RpcFunction::SetColor,
            RpcFunction::SetBrightness,
            RpcFunction::SetRegister,
        ];

        match command.function {
            RpcFunction::Subscribe => {}
            _ => {
                if functions_to_ok.contains(&command.function) {
                    self.0.push_back("R: Ok".to_string());
                } else {
                    self.0.push_back("R: 0".to_string());
                }
            }
        }
    }

    fn handle_command(&mut self, command: FtSwarmCommand) {
        match command {
            FtSwarmCommand::RPC(command) => { self.handle_rpc_command(command); }
            FtSwarmCommand::Direct(command) => { self.handle_direct_command(command); }
        }
    }
}

impl SwarmSerialPort for EmulatedSerialPort {
    fn available(&self) -> bool {
        !self.0.is_empty()
    }

    fn read_line(&mut self) -> String {
        self.0.pop_front().unwrap()
    }

    fn write_line(&mut self, line: String) {
        let command = FtSwarmCommand::deserialize(&line);
        match command {
            Ok(command) => {
                info!("Emulator received command: {:?}", command);
                self.handle_command(command);
            }
            Err(error) => {
                info!("Emulator received invalid command: {:?} (Command was {})", error, line);
            }
        }
    }

    fn block_until(&mut self, _line: String) {
        info!("Emulator has started")
    }
}