use std::io::Read;
use serialport::SerialPort;
use std::sync::Mutex;
use std::thread::sleep;

pub trait SwarmSerialPort : Send {
    fn available(&self) -> bool;
    fn read_line(&mut self) -> String;
    fn write_line(&mut self, line: String);
    fn block_until(&mut self, line: String);
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

impl SwarmSerialPort for SerialCommunication {
    fn available(&self) -> bool {
        self.port.bytes_to_read().unwrap_or(0) > 0
    }

    fn read_line(&mut self) -> String {
        let mut buffer = Vec::new();
        loop {
            let mut byte = [0];
            self.port.read_exact(&mut byte).unwrap();
            if byte[0] == b'\n' {
                break;
            }
            buffer.push(byte[0]);
        }
        String::from_utf8(buffer).unwrap()
    }

    fn write_line(&mut self, line: String) {
        self.port.write_all(line.as_bytes()).unwrap();
        self.port.write_all(b"\n").unwrap();
    }

    fn block_until(&mut self, line: String) {
        let mut buf = Vec::from(line.as_bytes());
        self.port.read_to_end(&mut buf).unwrap();
        sleep(std::time::Duration::from_millis(10));

        if let Ok(t) = self.port.bytes_to_read() {
            if t > 0 {
                let mut buf = vec![0; t as usize];
                self.port.read_exact(&mut buf).unwrap();
            }
        }
    }
}

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

    fn add_response(&self, response: String) {
        let mut commands = self.commands.lock().unwrap();
        commands.push(response);
    }

    fn pop_command(&self) -> Option<String> {
        let mut commands = self.commands.lock().unwrap();
        commands.pop()
    }

    fn initialize(&self) {
        let mut initialized = self.initialized.lock().unwrap();
        *initialized = true;
    }

    fn is_initialized(&self) -> bool {
        let initialized = self.initialized.lock().unwrap();
        *initialized
    }
}

impl SwarmSerialPort for FixedSerialPort {
    fn available(&self) -> bool {
        if self.is_initialized() {
            let commands = self.commands.lock().unwrap();
            !commands.is_empty()
        } else {
            false
        }
    }

    fn read_line(&mut self) -> String {
        let command = self.pop_command().unwrap();
        command
    }

    fn write_line(&mut self, line: String) {
        log::debug!("mock write line: {}", line)
    }

    fn block_until(&mut self, line: String) {
        log::debug!("mock block until: {}", line);
        self.initialize();
    }
}