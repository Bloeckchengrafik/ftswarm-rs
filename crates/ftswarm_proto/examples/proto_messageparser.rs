use ftswarm_proto::message_parser::S2RMessage;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;

fn main() {
    let messages = vec![
        "[2021-08-25 14:00:00] Log message",
        "R: RPC response",
        "S: Subscription response",
        "  ^ Error: message",
    ];

    for message in messages {
        match S2RMessage::try_from(message.to_string()) {
            Ok(msg) => println!("Message: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    }

    let return_values = vec![
        "ok",
        "1",
        "hello world"
    ];

    for value in return_values {
        match RPCReturnParam::try_from(value.to_string()) {
            Ok(msg) => println!("Param: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    }
}