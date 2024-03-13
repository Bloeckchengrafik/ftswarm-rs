

use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_int_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::SensorType;
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, NormallyOpen, SwarmObject, Updateable};

#[derive(Clone)]
pub struct Counter {
    pub name: String,
    pub value: i32,
    normally_open: NormallyOpen,
    swarm: FtSwarm
}

impl_int_updateable!(Counter);
impl_swarm_object!(Counter, NormallyOpen);

impl NewSwarmObject<NormallyOpen> for Counter {
    default_new_swarm_object_impls!();

    fn new(name: &str, swarm: FtSwarm, normally_open: NormallyOpen) -> Box<Self> {
        Box::new(Counter {
            name: name.to_string(),
            value: 0,
            normally_open,
            swarm
        })
    }

    fn init(&mut self) -> impl Future<Output = ()> {
        async move {
            self.run_command(
                RpcFunction::SetSensorType,
                vec![Argument::SensorType(SensorType::Counter), self.normally_open.clone().into()]
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
    
    