use std::{future::Future, sync::{Arc, Mutex}};

pub use analog::*;
pub use digital::*;
use ftswarm_macros::Updateable;
use ftswarm_proto::{command::{argument::Argument, FtSwarmCommand, rpc::{FtSwarmRPCCommand, RpcFunction}}, message_parser::rpc::RPCReturnParam};
pub use servo::Servo;

use crate::FtSwarm;

pub mod analog;
pub mod digital;

pub mod servo;

pub type Io<T> = Arc<Mutex<Box<T>>>;

pub trait Updateable {
    fn handle_subscription(&mut self, message: &RPCReturnParam);
}

pub trait NewSwarmObject<Params> {
    fn new(name: &str, swarm: FtSwarm, params: Params) -> Box<Self>;
    fn init(&mut self) -> impl Future<Output=()> {
        async move {}
    }
    fn name(&self) -> &str;
    fn swarm(&self) -> &FtSwarm;
}

pub trait SwarmObject<Params>: NewSwarmObject<Params> + Updateable + Clone + Sync + Send {
    fn create(swarm: &FtSwarm, name: &str, params: Params) -> impl Future<Output=Io<Self>> where Self: 'static {
        let obj = Self::new(name, swarm.clone(), params);
        let arc = Arc::new(Mutex::new(obj));
        let for_closure = arc.clone();
        swarm.push_cache(Box::new(move |subscription| {
            let mut obj = for_closure.lock().unwrap();
            obj.handle_subscription(&subscription);
        }), name);
        async move {
            {
                let mut obj = arc.lock().unwrap();
                obj.init().await;
            }
            arc
        }
    }

    fn run_command(&self, func: RpcFunction, args: Vec<Argument>) -> impl Future<Output=Result<RPCReturnParam, String>> {
        let command = FtSwarmRPCCommand {
            target: self.name().to_string(),
            function: func,
            args,
        };

        return self.swarm().transact(FtSwarmCommand::RPC(command));
    }
}

#[derive(Clone)]
pub struct Hysteresis(pub i32);


#[derive(Clone)]
pub enum NormallyOpen {
    Open,
    Closed,
}

impl Into<Argument> for NormallyOpen {
    fn into(self) -> Argument {
        match self {
            NormallyOpen::Open => Argument::Int(0),
            NormallyOpen::Closed => Argument::Int(1),
        }
    }
}