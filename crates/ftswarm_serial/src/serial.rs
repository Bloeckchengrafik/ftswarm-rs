use serialport::SerialPort;
use std::thread::sleep;
use std::io::{Read, Write};
use crate::SwarmSerialPort;

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
