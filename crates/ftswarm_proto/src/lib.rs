pub mod command;

pub trait IdOf {
    /// Some objects have an ID, this function returns it
    fn id(&self) -> u32;
}

pub trait NameOf {
    fn name(&self) -> String;
}

pub trait Serialized {
    /// Serialize the object to a string to be sent over the bus
    fn serialize(&self) -> String;
}
