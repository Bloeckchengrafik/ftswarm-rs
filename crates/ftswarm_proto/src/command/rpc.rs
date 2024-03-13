use crate::{NameOf, Serialized};
use crate::command::argument::Argument;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub enum RpcFunction {
    Show,
    TriggerUserEvent,
    SetMicroStepMode,
    GetMicroStepMode,
    Subscribe,
    GetIOType,
    SetSensorType,
    GetSensorType,
    GetValue,
    GetVoltage,
    GetResistance,
    GetKelvin,
    GetCelcius,
    GetFahrenheit,
    GetToggle,
    OnTrigger,
    GetActorType,
    SetActorType,
    SetSpeed,
    GetSpeed,
    SetMotionType,
    GetMotionType,
    SetPosition,
    GetPosition,
    SetOffset,
    GetOffset,
    SetColor,
    GetColor,
    SetBrightness,
    GetBrightness,
    SetRegister,
    GetRegister,
}
impl NameOf for RpcFunction {
    fn name(&self) -> String {
        match self {
            RpcFunction::Show => "show".to_string(),
            RpcFunction::TriggerUserEvent => "triggerUserEvent".to_string(),
            RpcFunction::SetMicroStepMode => "setMicroStepMode".to_string(),
            RpcFunction::GetMicroStepMode => "getMicroStepMode".to_string(),
            RpcFunction::Subscribe => "subscribe".to_string(),
            RpcFunction::GetIOType => "getIOType".to_string(),
            RpcFunction::SetSensorType => "setSensorType".to_string(),
            RpcFunction::GetSensorType => "getSensorType".to_string(),
            RpcFunction::GetValue => "getValue".to_string(),
            RpcFunction::GetVoltage => "getVoltage".to_string(),
            RpcFunction::GetResistance => "getResistance".to_string(),
            RpcFunction::GetKelvin => "getKelvin".to_string(),
            RpcFunction::GetCelcius => "getCelcius".to_string(),
            RpcFunction::GetFahrenheit => "getFahrenheit".to_string(),
            RpcFunction::GetToggle => "getToggle".to_string(),
            RpcFunction::OnTrigger => "onTrigger".to_string(),
            RpcFunction::GetActorType => "getActorType".to_string(),
            RpcFunction::SetActorType => "setActorType".to_string(),
            RpcFunction::SetSpeed => "setSpeed".to_string(),
            RpcFunction::GetSpeed => "getSpeed".to_string(),
            RpcFunction::SetMotionType => "setMotionType".to_string(),
            RpcFunction::GetMotionType => "getMotionType".to_string(),
            RpcFunction::SetPosition => "setPosition".to_string(),
            RpcFunction::GetPosition => "getPosition".to_string(),
            RpcFunction::SetOffset => "setOffset".to_string(),
            RpcFunction::GetOffset => "getOffset".to_string(),
            RpcFunction::SetColor => "setColor".to_string(),
            RpcFunction::GetColor => "getColor".to_string(),
            RpcFunction::SetBrightness => "setBrightness".to_string(),
            RpcFunction::GetBrightness => "getBrightness".to_string(),
            RpcFunction::SetRegister => "setRegister".to_string(),
            RpcFunction::GetRegister => "getRegister".to_string(),
        }
    }
}

pub struct FtSwarmRPCCommand {
    pub target: String,
    pub function: RpcFunction,
    pub args: Vec<Argument>,
}

impl Serialized for FtSwarmRPCCommand {
    fn serialize(&self) -> String {
        let mut serialized = format!("{}.{}(", self.target, self.function.name());

        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                serialized.push_str(", ");
            }
            serialized.push_str(&arg.serialize());
        }

        serialized.push_str(")");

        serialized
    }
}