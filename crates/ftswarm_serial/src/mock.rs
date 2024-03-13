use std::sync::Mutex;
use crate::SwarmSerialPort;

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

    pub fn add_response(&self, response: &str) {
        let mut commands = self.commands.lock().unwrap();
        commands.push(response.to_string());
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
