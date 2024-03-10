use ftswarm_proto::message_parser::ReturnMessageType;
use ftswarm_proto::message_parser::rpc::ReturnParam;

fn main() {
    let messages = vec![
        "[2021-08-25 14:00:00] Log message",
        "R: RPC response",
        "S: Subscription response",
        "  ^ Error: message",
    ];

    for message in messages {
        match ReturnMessageType::try_from(message.to_string()) {
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
        match ReturnParam::try_from(value.to_string()) {
            Ok(msg) => println!("Param: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    }
}