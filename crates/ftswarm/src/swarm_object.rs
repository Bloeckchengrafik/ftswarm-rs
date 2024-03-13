pub mod ntc;
pub mod analog;
pub mod voltmeter;
pub mod ohmmeter;
pub mod ldr;
pub mod trailsensor;
pub mod colorsensor;
pub mod ultrasonic;
pub mod digital;
pub mod counter;
pub mod frequencymeter;
pub mod lightbarrier;
pub mod reedswitch;
pub mod rotaryencoder;
pub mod switch;

pub mod servo;

use std::{future::Future, sync::{Arc, Mutex}};

use ftswarm_macros::Updateable;
use ftswarm_proto::{command::{argument::Argument, rpc::{FtSwarmRPCCommand, RpcFunction}, FtSwarmCommand}, message_parser::rpc::RPCReturnParam};

use crate::FtSwarm;

pub use servo::Servo;
pub use ntc::Ntc;
pub use analog::Analog;
pub use voltmeter::Voltmeter;
pub use ohmmeter::Ohmmeter;
pub use ldr::Ldr;
pub use trailsensor::TrailSensor;
pub use colorsensor::ColorSensor;
pub use ultrasonic::Ultrasonic;
pub use digital::Digital;
pub use counter::Counter;
pub use frequencymeter::FrequencyMeter;
pub use lightbarrier::LightBarrier;
pub use reedswitch::ReedSwitch;
pub use rotaryencoder::RotaryEncoder;
pub use switch::Switch;

pub type Io<T> = Arc<Mutex<Box<T>>>;

pub trait Updateable {
    fn handle_subscription(&mut self, message: &RPCReturnParam);
}

pub trait NewSwarmObject<Params> {
    fn new(name: &str, swarm: FtSwarm, params: Params) -> Box<Self>;
    fn init(&mut self) -> impl Future<Output = ()> {
        async move {}
    }
    fn name(&self) -> &str;
    fn swarm(&self) -> &FtSwarm;
}

pub trait SwarmObject<Params>: NewSwarmObject<Params> + Updateable + Clone + Sync + Send {
    fn create(swarm: &FtSwarm, name: &str, params: Params) -> impl Future<Output = Io<Self>> {
        let obj = Self::new(name, swarm.clone(), params);
        let arc = Arc::new(Mutex::new(obj));
        swarm.push_cache(arc.clone(), name);
        async move { 
            {
                let mut obj = arc.lock().unwrap();
                obj.init().await;
            }
            arc
        }
    }

    fn run_command(&self, func: RpcFunction, args: Vec<Argument>) -> impl Future<Output = Result<RPCReturnParam, String>> {
        let command = FtSwarmRPCCommand {
            target: self.name().to_string(),
            function: func,
            args
        };
        
        return self.swarm().transact(FtSwarmCommand::RPC(command));
    }
}

#[derive(Clone)]
pub struct Hysteresis(pub i32);