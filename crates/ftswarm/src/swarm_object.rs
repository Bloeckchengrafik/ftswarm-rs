use std::{future::Future, sync::Arc};

use ftswarm_macros::Updateable;
use ftswarm_proto::{command::{argument::Argument, rpc::{FtSwarmRPCCommand, RpcFunction}, FtSwarmCommand}, message_parser::rpc::RPCReturnParam};

use crate::{lock, FtSwarm, Mutex};

pub mod analog;
pub mod digital;

pub mod servo;
pub mod actor;
pub mod led;
pub mod controller;


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
    fn create(swarm: &FtSwarm, name: &str, params: Params) -> impl Future<Output=Io<Self>>
    where
        Self: 'static,
    {
        let obj = Self::new(name, swarm.clone(), params);
        let arc = Arc::new(Mutex::new(obj));
        let for_closure = arc.clone();

        async move {
            swarm.push_cache(Box::new(move |subscription| {
                let for_task = for_closure.clone();
                tokio::spawn(async move {
                    let mut obj = lock(&for_task).await;
                    obj.handle_subscription(&subscription);
                });
            }), name).await;
            {
                let mut obj = lock(&arc).await;
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

        self.swarm().transact(FtSwarmCommand::RPC(command))
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