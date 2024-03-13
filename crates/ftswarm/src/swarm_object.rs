use std::sync::{Arc, Mutex};

use ftswarm_macros::{SwarmObject, Updateable};
use ftswarm_proto::message_parser::rpc::RPCReturnParam;

use crate::FtSwarm;

pub trait Updateable {
    fn handle_subscription(&mut self, message: &RPCReturnParam);
}

pub trait NewSwarmObject {
    fn new(name: &str) -> Box<Self>;
}

pub trait SwarmObject: NewSwarmObject + Updateable + Clone + Sync + Send {
    fn create(swarm: &FtSwarm, name: &str) -> Arc<Mutex<Box<Self>>> {
        let obj = Self::new(name);
        let arc = Arc::new(Mutex::new(obj));
        swarm.push_cache(arc.clone(), name);
        arc
    }
}

#[derive(Updateable, SwarmObject, Clone)]
pub struct Servo {
    pub name: String
}

impl NewSwarmObject for Servo {
    fn new(name: &str) -> Box<Self> {
        Box::new(Servo {
            name: name.to_string()
        })
    }
}
