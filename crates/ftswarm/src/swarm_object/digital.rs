use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_bool_updateable};
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
digital_swarm_object!(RotaryEncoder, false);
digital_swarm_object!(LightBarrier, true);
digital_swarm_object!(ReedSwitch, true);
digital_swarm_object!(Switch, true);
