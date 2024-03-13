
use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_bool_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::{SensorType, ToggleType};
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, SwarmObject, Updateable};

#[derive(Clone)]
pub struct ReedSwitch {
    pub name: String,
    pub value: bool,
    swarm: FtSwarm,
}

impl_bool_updateable!(ReedSwitch);
impl_swarm_object!(ReedSwitch, ());

impl NewSwarmObject<()> for ReedSwitch {
    default_new_swarm_object_impls!();

    fn new(name: &str, swarm: FtSwarm, _: ()) -> Box<Self> {
        Box::new(ReedSwitch {
            name: name.to_string(),
            value: false,
            swarm,
        })
    }

    fn init(&mut self) -> impl Future<Output=()> {
        async move {
            self.run_command(
                RpcFunction::SetSensorType,
                vec![Argument::SensorType(SensorType::ReedSwitch)],
            ).await.unwrap();

            self.run_command(
                RpcFunction::Subscribe,
                vec![Argument::Int(0i64)],
            ).await.unwrap();

            self.value = self.run_command(RpcFunction::GetValue, vec![])
                .await.ok()
                .and_then(|param| param.as_int())
                .unwrap_or(0) == 1;
        }
    }
}

impl ReedSwitch {
    pub async fn get_toggle(&self) -> ToggleType {
        return self.run_command(RpcFunction::GetToggle, vec![])
            .await.ok()
            .and_then(|param| param.as_int())
            .and_then(|param| Some(ToggleType::from(param)))
            .unwrap_or(ToggleType::None);
    }
}

    