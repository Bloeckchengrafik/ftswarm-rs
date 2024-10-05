use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_bool_updateable, impl_int_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::SensorType;
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::proto::command::enums::ToggleType;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, NormallyOpen, SwarmObject, Updateable};
use ftswarm_macros::digital_swarm_object;

digital_swarm_object!(Digital, false);
digital_swarm_object!(FrequencyMeter, false);
digital_swarm_object!(Counter, false);
digital_swarm_object!(LightBarrier, true);
digital_swarm_object!(ReedSwitch, true);
digital_swarm_object!(Switch, true);

#[derive(Clone)]
pub struct RotaryEncoder {
    pub name: String,
    pub value: i32,
    should_subscribe: bool,
    normally_open: NormallyOpen,
    swarm: FtSwarm,
}
impl_int_updateable!(RotaryEncoder);
impl_swarm_object!(RotaryEncoder, bool );
impl NewSwarmObject<bool> for RotaryEncoder {
    default_new_swarm_object_impls!();
    fn new(name: &str, swarm: FtSwarm, should_subscribe: bool) -> Box<Self> {
        Box::new(RotaryEncoder { name: name.to_string(), value: 0, should_subscribe, normally_open: NormallyOpen::Closed, swarm })
    }

    fn init(&mut self) -> impl Future<Output=()> {
        async move {
            self.run_command(RpcFunction::SetSensorType, vec![Argument::SensorType(SensorType::RotaryEncoder), self.normally_open.clone().into()]).await.unwrap();
            if self.should_subscribe {
                self.run_command(RpcFunction::Subscribe, vec![Argument::Int(0i64)]).await.unwrap();
            }
            self.value = self.run_command(RpcFunction::GetValue, vec![]).await.ok().and_then(|param| param.as_int()).unwrap_or(0);
        }
    }
}
