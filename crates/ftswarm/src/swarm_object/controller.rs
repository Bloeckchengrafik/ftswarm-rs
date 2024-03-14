use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::MicroStepMode;
use ftswarm_proto::command::rpc::RpcFunction;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, SwarmObject, Updateable};

#[derive(Updateable, Clone)]
pub struct Controller {
    pub name: String,
    swarm: FtSwarm
}

impl_swarm_object!(Controller, ());

impl NewSwarmObject<()> for Controller {
    fn new(name: &str, swarm: FtSwarm, _params: ()) -> Box<Self> {
        Box::new(Controller {
            name: name.to_string(),
            swarm
        })
    }

    default_new_swarm_object_impls!();
}

impl Controller {
    pub async fn show(&self) -> Option<()> {
        self.run_command(RpcFunction::Show, vec![])
            .await.ok()
            .map(|_| ())
    }

    pub async fn set_micro_step_mode(&self, mode: MicroStepMode) -> Option<()> {
        self.run_command(RpcFunction::SetMicroStepMode, vec![Argument::MicroStepMode(mode)])
            .await.ok()
            .map(|_| ())
    }

    pub async fn set_register(&self, register: u8, value: u32) -> Option<()> {
        self.run_command(RpcFunction::SetRegister, vec![Argument::Int(register as i64), Argument::Int(value as i64)])
            .await.ok()
            .map(|_| ())
    }

    pub async fn get_register(&self, register: u8) -> Option<u32> {
        self.run_command(RpcFunction::GetRegister, vec![Argument::Int(register as i64)])
            .await.ok()
            .and_then(|response| response.as_int().map(|value| value as u32))
    }
}
