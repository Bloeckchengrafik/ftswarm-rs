use crate::{IdOf, Serialized};
use crate::command::enums::{ActorType, MotionType, SensorType};

pub enum Argument {
    Int(i64),
    Float(f64),
    ActorType(ActorType),
    SensorType(SensorType),
    MotionType(MotionType)
}

impl Serialized for Argument {
    fn serialize(&self) -> String {
        match self {
            Argument::Int(i) => i.to_string(),
            Argument::Float(f) => f.to_string(),
            Argument::ActorType(a) => a.id().to_string(),
            Argument::SensorType(s) => s.id().to_string(),
            Argument::MotionType(m) => m.id().to_string()
        }
    }
}