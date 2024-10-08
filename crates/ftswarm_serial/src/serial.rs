use serialport::SerialPort;
use std::thread::sleep;
use std::io::{Read, Write};
use log::trace;
use crate::{SerialError, SwarmSerialPort};

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
        let port = serialport::new(tty, 115200)
            .timeout(std::time::Duration::from_millis(10))
            .open()
            .unwrap_or_else(|_| panic!("Failed to open serial port at {}", tty));

        SerialCommunication {
            port
        }
    }

    pub fn get_first_available() -> Result<String, String> {
        let ports = serialport::available_ports()
            .map_err(|_| "No serial ports found")?;

        if ports.is_empty() {
            return Err("No serial ports found".to_string());
        }

        trace!("Found serial ports: {:?}", ports);

        Ok(ports[0].port_name.clone())
    }
}

impl Default for SerialCommunication {
    fn default() -> Self {
        let tty = SerialCommunication::get_first_available().expect("No serial ports found");
        SerialCommunication::connect(&tty)
    }
}

impl SwarmSerialPort for SerialCommunication {
    fn available(&self) -> Result<bool, SerialError> {
        Ok(self.port.bytes_to_read().map_err(|_| SerialError::IoError)? > 0)
    }

    fn read_line(&mut self) -> Result<String, SerialError> {
        let mut buffer = Vec::new();
        loop {
            let mut byte = [0];
            self.port.read_exact(&mut byte).map_err(|_| SerialError::IoError)?;
            if byte[0] == b'\n' {
                break;
            }
            buffer.push(byte[0]);
        }
        let str = String::from_utf8(buffer).map_err(|_| SerialError::EncodingError)?;
        trace!("S > R: {}", str);
        Ok(str)
    }

    fn write_line(&mut self, line: String) -> Result<(), SerialError> {
        trace!("R > S: {}", line);
        self.port.write_all(line.as_bytes()).map_err(|_| SerialError::IoError)?;
        self.port.write_all(b"\r\n").map_err(|_| SerialError::IoError)?;

        Ok(())
    }

    fn block_until(&mut self, line: String) -> Result<(), SerialError> {
        trace!("Blocking until: {}", line);
        // Read until the line is found
        let mut line_pos = 0;
        loop {
            if !self.available()? {
                sleep(std::time::Duration::from_millis(10));
                continue;
            }
            let mut byte = [0];
            self.port.read_exact(&mut byte).map_err(|_| SerialError::IoError)?;

            if byte[0] == line.as_bytes()[line_pos] {
                line_pos += 1;
                if line_pos == line.len() {
                    break;
                }
            } else {
                line_pos = 0;
            }
        }

        sleep(std::time::Duration::from_millis(10));

        if let Ok(t) = self.port.bytes_to_read() {
            if t > 0 {
                let mut buf = vec![0; t as usize];
                self.port.read_exact(&mut buf).map_err(|_| SerialError::IoError)?;
            }
        }

        trace!("Blocking until: {} - Done", line);
        Ok(())
    }
}
