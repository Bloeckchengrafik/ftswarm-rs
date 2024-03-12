use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};
use ftswarm_proto::command::direct::FtSwarmDirectCommand;
use ftswarm_proto::command::FtSwarmCommand;
use ftswarm_proto::message_parser::ReturnMessageType;
use ftswarm_proto::Serialized;
use ftswarm_serial::SwarmSerialPort;
use ftswarm_serial::serial::SerialCommunication;
use crate::message_queue::{ReturnQueue, WriteQueue};
use crate::swarm_object::SwarmObject;

mod message_queue;
pub mod swarm_object;

struct InnerFtSwarm {
    objects: HashMap<String, Box<dyn SwarmObject + Send>>,
    message_queue: ReturnQueue,
    write_queue: WriteQueue,
}

impl InnerFtSwarm {
    fn new() -> Self {
        InnerFtSwarm {
            objects: HashMap::new(),
            message_queue: ReturnQueue::new(),
            write_queue: WriteQueue::new(),
        }
    }
}

pub struct FtSwarm<Serial: SwarmSerialPort + 'static> {
    inner: Arc<Mutex<InnerFtSwarm>>,
    _serial: PhantomData<Serial>,
}

impl<Serial: SwarmSerialPort + 'static> FtSwarm<Serial> {
    pub fn new(mut serial: Serial) -> Self {
        let inner = Arc::new(Mutex::new(InnerFtSwarm::new()));

        let inner_for_thread = inner.clone();
        // Startup swarm serial mode

        serial.write_line(FtSwarmCommand::Direct(FtSwarmDirectCommand::StartCli).serialize());
        serial.block_until("@@@".to_string());

        tokio::spawn(async move {
            FtSwarm::input_loop(inner_for_thread, serial).await;
        });

        FtSwarm {
            inner,
            _serial: PhantomData,
        }
    }

    async fn input_loop(inner_ft_swarm: Arc<Mutex<InnerFtSwarm>>, mut serial_port: Serial) {
        loop {
            { // Free lock as soon as possible
                let mut inner = inner_ft_swarm.lock().unwrap();

                // Handle inputs
                if serial_port.available() {
                    let line = serial_port.read_line();
                    inner.message_queue.push(ReturnMessageType::from(line));
                }

                // Handle outputs
                if let Some(data) = inner.write_queue.pop() {
                    serial_port.write_line(data);
                }
            }

            sleep(Duration::from_millis(5)).await; // Don't be a menace to society while waiting
        }
    }
}

impl Default for FtSwarm<SerialCommunication> {
    fn default() -> Self {
        FtSwarm::new(SerialCommunication::default())
    }
}