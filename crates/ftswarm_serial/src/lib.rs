pub use mock::FixedSerialPort;
pub use serial::SerialCommunication;

pub mod serial;
pub mod mock;

pub trait SwarmSerialPort: Send {
    fn available(&self) -> bool;
    fn read_line(&mut self) -> String;
    fn write_line(&mut self, line: String);
    fn block_until(&mut self, line: String);
}
