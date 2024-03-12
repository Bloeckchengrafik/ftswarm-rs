use ftswarm_proto::message_parser::rpc::RPCReturnParam;

pub trait SwarmObject {
    fn handle_subscription(&mut self, message: &RPCReturnParam);
}
