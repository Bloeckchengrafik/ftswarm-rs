use crate::{Deserialized, IdOf, Serialized};
use crate::command::enums::{ActorType, MicroStepMode, MotionType, SensorType};

#[derive(Debug, Clone)]
pub enum Argument {
    Int(i64),
    Float(f64),
    Bool(bool),
    ActorType(ActorType),
    SensorType(SensorType),
    MotionType(MotionType),
    MicroStepMode(MicroStepMode)
}

impl Serialized for Argument {
    fn serialize(&self) -> String {
        match self {
            Argument::Int(i) => i.to_string(),
            Argument::Float(f) => f.to_string(),
            Argument::Bool(b) => (if b.clone() { 1 } else { 0 }).to_string(),
            Argument::ActorType(a) => a.id().to_string(),
            Argument::SensorType(s) => s.id().to_string(),
            Argument::MotionType(m) => m.id().to_string(),
            Argument::MicroStepMode(m) => m.id().to_string()
        }
    }
}

impl Deserialized for Argument {
    fn deserialize(value: &String) -> Result<Self, String> where Self: Sized {
        Ok(Argument::Int(value.parse::<i64>().map_err(|_| format!("Error parsing int at {}", value))?))
    }
}