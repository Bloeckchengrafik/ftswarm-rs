use crate::{IdOf, Serialized};
use crate::command::enums::{ActorType, MotionType, SensorType};

pub enum Argument {
    Int(i64),
    Float(f64),
    Bool(bool),
    ActorType(ActorType),
    SensorType(SensorType),
    MotionType(MotionType)
}

impl Serialized for Argument {
    fn serialize(&self) -> String {
        match self {
            Argument::Int(i) => i.to_string(),
            Argument::Float(f) => f.to_string(),
            Argument::Bool(b) => (if b.clone() { 1 } else { 0 }).to_string(),
            Argument::ActorType(a) => a.id().to_string(),
            Argument::SensorType(s) => s.id().to_string(),
            Argument::MotionType(m) => m.id().to_string()
        }
    }
}