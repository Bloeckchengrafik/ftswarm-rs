use std::sync::Mutex;
use crate::{SerialError, SwarmSerialPort};

pub struct FixedSerialPort {
    commands: Mutex<Vec<String>>,
    initialized: Mutex<bool>,
}

impl Default for FixedSerialPort {
    fn default() -> Self {
        Self::new()
    }
}

impl FixedSerialPort {
    pub fn new() -> Self {
        FixedSerialPort {
            commands: Mutex::new(Vec::new()),
            initialized: Mutex::new(false),
        }
    }

    pub fn add_response(&self, response: &str) {
        let mut commands = self.commands.lock().unwrap();
        commands.insert(0, response.to_string());
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
    fn available(&self) -> Result<bool, SerialError> {
        if self.is_initialized() {
            let commands = self.commands.lock().map_err(|_| SerialError::Other("Mutex error".to_string()))?;
            Ok(!commands.is_empty())
        } else {
            Ok(false)
        }
    }

    fn read_line(&mut self) -> Result<String, SerialError> {
        let command = self.pop_command().ok_or(SerialError::Timeout)?;
        Ok(command)
    }

    fn write_line(&mut self, line: String) -> Result<(), SerialError> {
        log::debug!("mock write line: {}", line);
        Ok(())
    }

    fn block_until(&mut self, line: String) -> Result<(), SerialError> {
        log::debug!("mock block until: {}", line);
        self.initialize();

        Ok(())
    }
}
