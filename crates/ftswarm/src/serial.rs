use serialport::SerialPort;
use tokio::sync::Mutex;

pub trait SwarmSerialPort : Send {
}

pub struct SerialCommunication {
    port: Box<dyn SerialPort>,
}

impl SerialCommunication {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        SerialCommunication {
            port
        }
    }

    pub fn connect(tty: &str) -> Self {
        let port = serialport::new(tty, 115200).open().unwrap();
        SerialCommunication {
            port
        }
    }
}

impl Default for SerialCommunication {
    fn default() -> Self {
        let tty = serialport::available_ports().expect("No serial ports found").first().unwrap().port_name.clone();
        SerialCommunication::connect(&tty)
    }
}

impl SwarmSerialPort for SerialCommunication {}

pub struct FixedSerialPort {
    commands: Mutex<Vec<String>>,
    initialized: Mutex<bool>,
}

impl FixedSerialPort {
    pub fn new() -> Self {
        FixedSerialPort {
            commands: Mutex::new(Vec::new()),
            initialized: Mutex::new(false),
        }
    }

    async fn add_response(&self, response: String) {
        let mut commands = self.commands.lock().await;
        commands.push(response);
    }

    async fn pop_command(&self) -> Option<String> {
        let mut commands = self.commands.lock().await;
        commands.pop()
    }

    async fn initialize(&self) {
        let mut initialized = self.initialized.lock().await;
        *initialized = true;
    }

    async fn is_initialized(&self) -> bool {
        let initialized = self.initialized.lock().await;
        *initialized
    }
}

impl SwarmSerialPort for FixedSerialPort {}