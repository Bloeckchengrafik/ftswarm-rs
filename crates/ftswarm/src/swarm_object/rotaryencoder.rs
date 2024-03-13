

use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_int_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::SensorType;
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, SwarmObject, Updateable};

#[derive(Clone)]
pub struct RotaryEncoder {
    pub name: String,
    pub value: i32,
    swarm: FtSwarm
}

impl_int_updateable!(RotaryEncoder);
impl_swarm_object!(RotaryEncoder, ());

impl NewSwarmObject<()> for RotaryEncoder {
    default_new_swarm_object_impls!();

    fn new(name: &str, swarm: FtSwarm, _: ()) -> Box<Self> {
        Box::new(RotaryEncoder {
            name: name.to_string(),
            value: 0,
            swarm
        })
    }

    fn init(&mut self) -> impl Future<Output = ()> {
        async move {
            self.run_command(
                RpcFunction::SetSensorType,
                vec![Argument::SensorType(SensorType::RotaryEncoder)]
            ).await.unwrap();

            self.run_command(
                RpcFunction::Subscribe,
                vec![Argument::Int(0i64)]
            ).await.unwrap();

            self.value = self.run_command(RpcFunction::GetValue, vec![])
                .await.ok()
                .and_then(|param| param.as_int())
                .unwrap_or(0);
        }
    }
}
    
    