use super::rpc::RPCReturnParam;

pub struct Subscription {
    pub port_name: String,
    pub value: RPCReturnParam,
}

impl TryFrom<String> for Subscription {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // PortName Value
        let mut parts = value.split_whitespace();
        let port_name = parts.next().ok_or(())?.to_string();
        let value = parts.next().ok_or(())?.to_string();
        let value = RPCReturnParam::from(value);

        Ok(Subscription { port_name, value })
    }
}