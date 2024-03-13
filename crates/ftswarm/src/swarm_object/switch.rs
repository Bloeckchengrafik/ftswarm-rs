
use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_bool_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::{SensorType, ToggleType};
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, NormallyOpen, SwarmObject, Updateable};

#[derive(Clone)]
pub struct Switch {
    pub name: String,
    pub value: bool,
    pub normally_open: NormallyOpen,
    swarm: FtSwarm,
}

impl_bool_updateable!(Switch);
impl_swarm_object!(Switch, NormallyOpen);

impl NewSwarmObject<NormallyOpen> for Switch {
    default_new_swarm_object_impls!();

    fn new(name: &str, swarm: FtSwarm, normally_open: NormallyOpen) -> Box<Self> {
        Box::new(Switch {
            name: name.to_string(),
            value: false,
            normally_open,
            swarm,
        })
    }

    fn init(&mut self) -> impl Future<Output=()> {
        async move {
            self.run_command(
                RpcFunction::SetSensorType,
                vec![Argument::SensorType(SensorType::Switch), self.normally_open.clone().into()],
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

impl Switch {
    pub async fn get_toggle(&self) -> ToggleType {
        return self.run_command(RpcFunction::GetToggle, vec![])
            .await.ok()
            .and_then(|param| param.as_int())
            .and_then(|param| Some(ToggleType::from(param)))
            .unwrap_or(ToggleType::None);
    }
}

    