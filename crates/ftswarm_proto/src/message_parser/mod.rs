pub mod rpc;
pub mod subscription;

#[derive(Debug, Clone)]
pub enum S2RMessage {
    Log(String),
    RPCResponse(String),
    Subscription(String),
    Error(String),
    StartCLI
}

fn is_log_message(message: &str) -> bool {
    message.starts_with("[")
}

fn is_rpc_response(message: &str) -> bool {
    message.starts_with("R: ")
}

fn is_subscription_response(message: &str) -> bool {
    message.starts_with("S: ")
}

fn is_error_message(message: &str) -> bool {
    message.trim().starts_with("^")
}

impl From<String> for S2RMessage {
    fn from(value: String) -> Self {
        if value.contains("@@@ ftSwarmOS CLI started") {
            return S2RMessage::StartCLI;
        }

        return if is_log_message(&value) {
            S2RMessage::Log(value)
        } else if is_rpc_response(&value) {
            S2RMessage::RPCResponse(value.replacen("R: ", "", 1))
        } else if is_subscription_response(&value) {
            S2RMessage::Subscription(value.replacen("S: ", "", 1))
        } else if is_error_message(&value) {
            S2RMessage::Error(value.replacen("^ Error:", "", 1).trim().to_string())
        } else {
            S2RMessage::RPCResponse(value)
        }
    }
}
