use std::any::type_name;
use std::collections::HashMap;
use std::io::read_to_string;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};
use ftswarm_proto::command::direct::FtSwarmDirectCommand;
use ftswarm_proto::command::FtSwarmCommand;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use ftswarm_proto::message_parser::S2RMessage;
use ftswarm_proto::message_parser::S2RMessage::Subscription;
use ftswarm_proto::Serialized;
use ftswarm_serial::SwarmSerialPort;
use ftswarm_serial::serial::SerialCommunication;
use crate::message_queue::{ReturnQueue, SenderHandle, WriteQueue};
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
    coro: JoinHandle<()>,
    _serial: PhantomData<Serial>,
}

impl<Serial: SwarmSerialPort + 'static> FtSwarm<Serial> {
    pub fn new(mut serial: Serial) -> Self {
        let inner = Arc::new(Mutex::new(InnerFtSwarm::new()));

        let inner_for_thread = inner.clone();
        // Startup swarm serial mode

        serial.write_line(FtSwarmCommand::Direct(FtSwarmDirectCommand::StartCli).serialize());
        serial.block_until("@@@".to_string());

        let handle = tokio::spawn(async move {
            FtSwarm::input_loop(inner_for_thread, serial).await;
            ()
        });

        FtSwarm {
            inner,
            coro: handle,
            _serial: PhantomData,
        }
    }

    async fn input_loop(inner_ft_swarm: Arc<Mutex<InnerFtSwarm>>, mut serial_port: Serial) {
        loop {
            { // Free lock as soon as possible
                let mut inner = inner_ft_swarm.lock().unwrap();

                // Handle inputs
                if serial_port.available() {
                    let line = serial_port.read_line().replace("\n", "").replace("\r", "");
                    let response = S2RMessage::from(line);
                    if let Subscription(subscription) = response {
                        dbg!(subscription);
                    } else {
                        inner.message_queue.push(response);
                    }
                }

                // Handle outputs
                if let Some(data) = inner.write_queue.pop() {
                    serial_port.write_line(data);
                }
            }

            sleep(Duration::from_millis(5)).await; // Don't be a menace to society while waiting
        }
    }

    pub fn send_command(&self, command: FtSwarmCommand) {
        let mut inner = self.inner.lock().unwrap();
        inner.write_queue.push(command);
    }

    pub async fn read_response(&self) -> Result<RPCReturnParam, String> {
        let (handle, mut recv) = SenderHandle::create();
        {
            let mut inner = self.inner.lock().unwrap();
            inner.message_queue.push_sender(&handle);
        }

        let response = recv.recv().await.unwrap();

        {
            let mut inner = self.inner.lock().unwrap();
            inner.message_queue.drop_sender(&handle);
        }

        match response {
            S2RMessage::RPCResponse(data) => Ok(RPCReturnParam::from(data)),
            S2RMessage::Error(data) => Err(data),
            _ => Err("Received non-RPCResponse message".to_string()),
        }
    }
}

// RAII this out of existence
impl<Serial: SwarmSerialPort + 'static> Drop for FtSwarm<Serial> {
    fn drop(&mut self) {
        self.coro.abort();
    }
}

impl Default for FtSwarm<SerialCommunication> {
    fn default() -> Self {
        FtSwarm::new(SerialCommunication::default())
    }
}