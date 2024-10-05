pub use mock::FixedSerialPort;
pub use serial::SerialCommunication;

pub mod serial;
pub mod mock;

#[derive(Debug)]
pub enum SerialError {
    IoError,
    Timeout,
    ManualDisconnect,
    EncodingError,
    Other(String),
}

pub trait SwarmSerialPort: Send {
    fn available(&self) -> Result<bool, SerialError>;
    fn read_line(&mut self) -> Result<String, SerialError>;
    fn write_line(&mut self, line: String) -> Result<(), SerialError>;
    fn block_until(&mut self, line: String) -> Result<(), SerialError>;
}
