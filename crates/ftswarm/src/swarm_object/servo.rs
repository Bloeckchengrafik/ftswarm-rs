use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::rpc::RpcFunction;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, SwarmObject, Updateable};

#[derive(Updateable, Clone)]
pub struct Servo {
    pub name: String,
    swarm: FtSwarm
}

impl_swarm_object!(Servo, ());

impl NewSwarmObject<()> for Servo {
    fn new(name: &str, swarm: FtSwarm, _params: ()) -> Box<Self> {
        Box::new(Servo {
            name: name.to_string(),
            swarm
        })
    }

    default_new_swarm_object_impls!();
}

impl Servo {
    pub async fn get_position(&self) -> Option<i32> {
        self.run_command(RpcFunction::GetPosition, vec![])
            .await.ok()
            .and_then(|param| param.as_int())
    }

    pub async fn set_position(&self, position: i32) -> Option<()> {
        self.run_command(RpcFunction::SetPosition, vec![Argument::Int(position as i64)])
            .await.ok()
            .map(|_| ())
    }

    pub async fn get_offset(&self) -> Option<i32> {
        self.run_command(RpcFunction::GetOffset, vec![])
            .await.ok()
            .and_then(|param| param.as_int())
    }

    pub async fn set_offset(&self, offset: i32) -> Option<()> {
        self.run_command(RpcFunction::SetOffset, vec![Argument::Int(offset as i64)])
            .await.ok()
            .map(|_| ())
    }
}
