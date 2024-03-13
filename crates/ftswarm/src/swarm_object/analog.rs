
use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_int_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::SensorType;
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::FtSwarm;
use crate::swarm_object::{Hysteresis, NewSwarmObject, SwarmObject, Updateable};

#[derive(Clone)]
pub struct Analog {
    pub name: String,
    pub hysteresis: Hysteresis,
    pub value: i32,
    swarm: FtSwarm
}

impl_int_updateable!(Analog);
impl_swarm_object!(Analog, Hysteresis);

impl NewSwarmObject<Hysteresis> for Analog {
    default_new_swarm_object_impls!();

    fn new(name: &str, swarm: FtSwarm, hysteresis: Hysteresis) -> Box<Self> {
        Box::new(Analog {
            name: name.to_string(),
            hysteresis,
            value: 0,
            swarm
        })
    }

    fn init(&mut self) -> impl Future<Output = ()> {
        async move {
            self.run_command(
                RpcFunction::SetSensorType,
                vec![Argument::SensorType(SensorType::Analog)]
            ).await.unwrap();

            self.run_command(
                RpcFunction::Subscribe,
                vec![Argument::Int(self.hysteresis.0.clone() as i64)]
            ).await.unwrap();

            self.value = self.run_command(RpcFunction::GetValue, vec![])
                .await.ok()
                .and_then(|param| param.as_int())
                .unwrap_or(0);
        }
    }
}
    