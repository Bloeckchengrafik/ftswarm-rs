use std::{future::{Future, IntoFuture}, sync::{Arc, Mutex}};

use ftswarm_macros::Updateable;
use ftswarm_proto::{command::{argument::Argument, rpc::{FtSwarmRPCCommand, RpcFunction}, FtSwarmCommand}, message_parser::rpc::RPCReturnParam};

use crate::FtSwarm;

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

        if func.clone() == RpcFunction::Subscribe {
            self.swarm().send_command(FtSwarmCommand::RPC(command));

            return async move {
                Ok(RPCReturnParam::Ok)
            }
        }
        
        let fut = self.swarm().transact(FtSwarmCommand::RPC(command));

        return fut;
    }
}

#[derive(Updateable, Clone)]
pub struct Servo {
    pub name: String,
    swarm: FtSwarm
}

impl SwarmObject<()> for Servo {
    
}

impl NewSwarmObject<()> for Servo {
    fn new(name: &str, swarm: FtSwarm, _params: ()) -> Box<Self> {
        Box::new(Servo {
            name: name.to_string(),
            swarm
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn swarm(&self) -> &FtSwarm {
        &self.swarm
    }
}

impl Servo {
    pub async fn get_position(&self) -> Option<i32> {
        self.run_command(RpcFunction::GetPosition, vec![])
            .await.ok()
            .and_then(|param| param.as_int())
    }

    pub async fn set_position(&self, position: i32) -> Result<(), String> {
        self.run_command(RpcFunction::SetPosition, vec![Argument::Int(position as i64)])
            .await
            .map(|_| ())
    }

    pub async fn get_offset(&self) -> Option<i32> {
        self.run_command(RpcFunction::GetOffset, vec![])
            .await.ok()
            .and_then(|param| param.as_int())
    }

    pub async fn set_offset(&self, offset: i32) -> Result<(), String> {
        self.run_command(RpcFunction::SetOffset, vec![Argument::Int(offset as i64)])
            .await
            .map(|_| ())
    }
}

#[derive(Clone)]
pub struct Ntc {
    pub name: String,
    pub hysteresis: Hysteresis,
    pub value: i32,
    swarm: FtSwarm
}

#[derive(Clone)]
pub struct Hysteresis(pub i32);

impl SwarmObject<Hysteresis> for Ntc {
}

impl Updateable for Ntc {
    fn handle_subscription(&mut self, message: &RPCReturnParam) {
        if let RPCReturnParam::Int(value) = message {
            self.value = *value;
        }
    }
}

impl NewSwarmObject<Hysteresis> for Ntc {
    fn new(name: &str, swarm: FtSwarm, hysteresis: Hysteresis) -> Box<Self> {
        Box::new(Ntc {
            name: name.to_string(),
            hysteresis,
            value: 0,
            swarm
        })
    }

    fn init(&mut self) -> impl Future<Output = ()> {
        async move {
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

    fn name(&self) -> &str {
        &self.name
    }

    fn swarm(&self) -> &FtSwarm {
        &self.swarm
    }
}