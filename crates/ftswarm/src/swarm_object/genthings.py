def filename_from_name(struct_name):
    return struct_name.lower() + ".rs"


def genfile(gen_for_name):
    filename = filename_from_name(gen_for_name)

    contents = """

use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_int_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::SensorType;
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, SwarmObject, Updateable};

#[derive(Clone)]
pub struct Ultrasonic {
    pub name: String,
    pub value: i32,
    swarm: FtSwarm
}

impl_int_updateable!(#NAME#);
impl_swarm_object!(#NAME#, ());

impl NewSwarmObject<()> for #NAME# {
    default_new_swarm_object_impls!();

    fn new(name: &str, swarm: FtSwarm, _: ()) -> Box<Self> {
        Box::new(Ultrasonic {
            name: name.to_string(),
            value: 0,
            swarm
        })
    }

    fn init(&mut self) -> impl Future<Output = ()> {
        async move {
            self.run_command(
                RpcFunction::SetSensorType,
                vec![Argument::SensorType(SensorType::#NAME#)]
            ).await.unwrap();

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
}
    
    """.replace("#NAME#", gen_for_name)

    with open(filename, "w") as f:
        f.write(contents)


if __name__ == '__main__':
    gen_for = [
        "Counter",
        "RotaryEncoder",
        "FrequencyMeter",
    ]

    for name in gen_for:
        genfile(name)
