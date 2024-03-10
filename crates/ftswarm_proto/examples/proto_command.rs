use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::direct::FtSwarmDirectCommand;
use ftswarm_proto::command::enums::ActorType::Compressor;
use ftswarm_proto::command::enums::MotionType::On;
use ftswarm_proto::command::enums::SensorType::Digital;
use ftswarm_proto::command::FtSwarmCommand;
use ftswarm_proto::command::rpc::{FtSwarmRPCCommand, RpcFunction};
use ftswarm_proto::Serialized;

fn main() {
    let direct_command = FtSwarmCommand::Direct(FtSwarmDirectCommand::Whoami);
    let rpc_command = FtSwarmCommand::RPC(FtSwarmRPCCommand {
        target: "test".to_string(),
        function: RpcFunction::GetFahrenheit,
        args: vec![
            Argument::Int(32),
            Argument::Float(32.0),
            Argument::SensorType(Digital),
            Argument::ActorType(Compressor),
            Argument::MotionType(On)
        ],
    });

    println!("Direct command: {:?}", direct_command.serialize());
    println!("RPC command: {:?}", rpc_command.serialize());
}