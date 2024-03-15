pub mod command;
pub mod message_parser;

pub trait IdOf {
    /// Some objects have an ID, this function returns it
    fn id(&self) -> u32;
}

pub trait NameOf {
    fn name(&self) -> String;
}

pub trait Serialized {
    /// Serialize the object to a string to be sent over the bus
    fn serialize(&self) -> String;
}

pub trait Deserialized {
    /// Deserialize the string into the object
    fn deserialize(value: &String) -> Result<Self, String> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use crate::command::argument::Argument;
    use crate::command::direct::FtSwarmDirectCommand::Help;
    use crate::command::FtSwarmCommand;
    use crate::command::rpc::FtSwarmRPCCommand;
    use crate::command::rpc::RpcFunction::GetResistance;
    use crate::{Deserialized, Serialized};

    fn test_serialize<T: Serialized>(obj: T, expected: &str) {
        assert_eq!(obj.serialize(), expected);
    }

    #[test]
    fn test_serialize_rpc() {
        test_serialize(FtSwarmCommand::RPC(FtSwarmRPCCommand {
            target: "hello".to_string(),
            function: GetResistance,
            args: vec![Argument::Int(42)]
        }), "hello.getResistance(42)");
    }

    #[test]
    fn test_serialize_direct() {
        test_serialize(FtSwarmCommand::Direct(Help), "help");
    }

    #[test]
    fn test_deserialize_rpc() {
        let cmd = FtSwarmCommand::deserialize(&"hello.getResistance(42)".to_string()).unwrap();
        match cmd {
            FtSwarmCommand::RPC(rpc) => {
                assert_eq!(rpc.target, "hello");
                assert_eq!(rpc.function, GetResistance);
                assert_eq!(rpc.args.len(), 1);
                match &rpc.args[0] {
                    Argument::Int(i) => assert_eq!(*i, 42),
                    _ => panic!("Expected int argument")
                }
            },
            _ => panic!("Expected RPC command")
        }
    }

    #[test]
    fn test_deserialize_direct() {
        let cmd = FtSwarmCommand::deserialize(&"help".to_string()).unwrap();
        match cmd {
            FtSwarmCommand::Direct(Help) => {},
            _ => panic!("Expected Direct command")
        }
    }
}