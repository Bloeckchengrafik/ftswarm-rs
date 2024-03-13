use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use proto::message_parser::subscription::Subscription;
use swarm_object::Updateable;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};
use ftswarm_proto::command::direct::FtSwarmDirectCommand;
use ftswarm_proto::command::FtSwarmCommand;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use ftswarm_proto::message_parser::S2RMessage;
use ftswarm_proto::Serialized;
use ftswarm_serial::SwarmSerialPort;
use ftswarm_serial::serial::SerialCommunication;
use crate::message_queue::{ReturnQueue, SenderHandle, WriteQueue};

pub use ftswarm_proto as proto;
use ftswarm_proto::command::rpc::RpcFunction;
use crate::direct::{parse_uptime, WhoamiResponse};

mod message_queue;
pub mod swarm_object;
mod direct;

#[cfg(test)]
mod tests;

/// A macro to create a struct with static string aliases
///
/// # Example
///
/// ```
/// use ftswarm::aliases;
///
/// aliases! {
///     Aliases {
///         SWITCH = "switch",
///         LED1 = "led1",
///         LED2 = "led2",
///     }
/// }
///
/// fn main() {
///    println!("Switch alias: {}", Aliases::SWITCH);
/// }
/// ```
///
/// This is useful for creating type-safe alias names for ftSwarm objects
#[macro_export]
macro_rules! aliases {
    (
        $enum_name:ident {
            $(
                $variant:ident = $alias:expr
            ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub struct $enum_name {
        }

        impl $enum_name {
            $(
                pub const $variant: &'static str = $alias;
            )*
        }
    };
}

struct InnerFtSwarm {
    objects: HashMap<String, Arc<Mutex<Box<dyn Updateable + Send + Sync>>>>,
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

/// A struct representing a connection to an ftSwarm
pub struct FtSwarm {
    inner: Arc<Mutex<InnerFtSwarm>>,
    coro: Option<JoinHandle<()>>,
}

impl FtSwarm {
    /// Create a new FtSwarm instance, you must provide a serial port to connect to it
    pub fn new<Serial: SwarmSerialPort + 'static>(mut serial: Serial) -> Self {
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
            coro: Some(handle),
        }
    }

    async fn input_loop<Serial: SwarmSerialPort + 'static>(inner_ft_swarm: Arc<Mutex<InnerFtSwarm>>, mut serial_port: Serial) {
        loop {
            { // Free lock as soon as possible
                let mut inner = inner_ft_swarm.lock().unwrap();

                // Handle inputs
                if serial_port.available() {
                    let line = serial_port.read_line().replace("\n", "").replace("\r", "");
                    let response = S2RMessage::from(line);
                    if let S2RMessage::Subscription(subscription) = response {
                        if let Ok(subscription) = Subscription::try_from(subscription) {
                            if let Some(object) = inner.objects.get(&subscription.port_name) {
                                let mut object = object.lock().unwrap();
                                object.handle_subscription(&subscription.value);
                            }
                        }   
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

    pub(crate) fn push_cache<T: Updateable + Send + Sync>(&self, object: Arc<Mutex<Box<T>>>, name: &str) {
        let mut inner = self.inner.lock().unwrap();
        inner.objects.insert(name.to_string(), object);
    }

    /// Low-level method to send a command to the ftSwarm. Only use this as a last resort
    pub fn send_command(&self, command: FtSwarmCommand) {
        let mut inner = self.inner.lock().unwrap();
        inner.write_queue.push(command);
    }

    /// Low-level method to receive a response to the ftSwarm. Only use this as a last resort
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


    /// Low-level method to send a command to the ftSwarm and receive a response. Only use this as a last resort
    pub async fn transact(&self, command: FtSwarmCommand) -> Result<RPCReturnParam, String> {
        // Subscribe commands don't return a response
        let is_subscription = match &command {
            FtSwarmCommand::RPC(cmd) => cmd.function == RpcFunction::Subscribe,
            _ => false,
        };

        self.send_command(command);

        if is_subscription {
            return Ok(RPCReturnParam::Ok)
        }

        self.read_response().await
    }

    /// Return the hostname, id, and serial number of the connected ftSwarm
    pub async fn whoami(&self) -> Result<WhoamiResponse, String> {
        let response = self.transact(FtSwarmCommand::Direct(FtSwarmDirectCommand::Whoami)).await?;
        if let RPCReturnParam::String(str) = response {
            Ok(WhoamiResponse::try_from(str)?)
        } else {
            Err("Received non-string response".to_string())
        }
    }

    /// Stop all connected motors and turn off all LEDs (except for RGB LEDs)
    pub fn halt(&self) {
        self.send_command(FtSwarmCommand::Direct(FtSwarmDirectCommand::Halt));
    }

    /// Return the uptime of the connected ftSwarm (max precision: seconds)
    pub async fn uptime(&self) -> Result<Duration, String> {
        let response = self.transact(FtSwarmCommand::Direct(FtSwarmDirectCommand::Uptime)).await?;

        if let RPCReturnParam::String(str) = response {
            Ok(parse_uptime(str)?)
        } else {
            Err("Received non-string response".to_string())
        }
    }
}

impl Drop for FtSwarm {
    fn drop(&mut self) {
        if let Some(coro) = self.coro.take() {
            coro.abort();
        }
    }
}

impl Clone for FtSwarm {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), coro: None }
    }
}

/// You can use the default implementation of FtSwarm if you just want to connect to the
/// first available ftSwarm
impl Default for FtSwarm {
    fn default() -> Self {
        FtSwarm::new(SerialCommunication::default())
    }
}