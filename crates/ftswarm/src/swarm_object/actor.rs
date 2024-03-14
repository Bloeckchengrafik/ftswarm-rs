use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::ActorType;
use ftswarm_proto::command::rpc::RpcFunction;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, SwarmObject, Updateable};
use ftswarm_macros::actor_swarm_object;


pub enum ValueState {
    High,
    Reverse,
    Low,
}

impl From<bool> for ValueState {
    fn from(value: bool) -> Self {
        if value {
            ValueState::High
        } else {
            ValueState::Low
        }
    }
}

impl Into<i64> for ValueState {
    fn into(self) -> i64 {
        match self {
            ValueState::High => 255,
            ValueState::Reverse => -255,
            ValueState::Low => 0,
        }
    }
}

actor_swarm_object!(Motor, false);
actor_swarm_object!(XMMotor, false);
actor_swarm_object!(Tractor, false);
actor_swarm_object!(Encoder, false);
actor_swarm_object!(Lamp, true);
actor_swarm_object!(Valve, true);
actor_swarm_object!(Compressor, true);
actor_swarm_object!(Buzzer, true);
