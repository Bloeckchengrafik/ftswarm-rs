use std::collections::HashMap;
use std::sync::Arc;

#[cfg(not(feature = "tokio_mutex"))]
use std::sync::Mutex as StdMutex;

#[cfg(feature = "tokio_mutex")]
use tokio::sync::Mutex as TokioMutex;

use proto::message_parser::subscription::Subscription;
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
pub mod prelude;

#[cfg(test)]
mod tests;

// Set mutex type based on the feature flag
#[cfg(feature = "tokio_mutex")]
pub type Mutex<T> = TokioMutex<T>;
#[cfg(not(feature = "tokio_mutex"))]
pub type Mutex<T> = StdMutex<T>;

#[cfg(feature = "tokio_mutex")]
async fn lock<T>(mutex: &Mutex<T>) -> tokio::sync::MutexGuard<T> {
    mutex.lock().await
}

#[cfg(not(feature = "tokio_mutex"))]
async fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<T> {
    mutex.lock().unwrap()
}

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
/// println!("Switch alias: {}", Aliases::SWITCH);
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
    objects: HashMap<String, Box<dyn Fn(RPCReturnParam) + Send>>,
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

        serial.write_line(FtSwarmCommand::Direct(FtSwarmDirectCommand::StartCli).serialize()).expect("Write line failure");
        serial.block_until("@@@".to_string()).expect("Block until failure");

        let handle = tokio::spawn(async move {
            FtSwarm::input_loop(inner_for_thread, serial).await;
        });

        FtSwarm {
            inner,
            coro: Some(handle),
        }
    }

    async fn input_loop<Serial: SwarmSerialPort + 'static>(inner_ft_swarm: Arc<Mutex<InnerFtSwarm>>, mut serial_port: Serial) {
        loop {
            if serial_port.available().expect("Available check failure") {
                let line = serial_port.read_line().expect("Readline failure").replace("\n", "").replace("\r", "");
                let response = S2RMessage::from(line);
                {
                    let mut inner = lock(&inner_ft_swarm).await;
                    if let S2RMessage::Subscription(subscription) = response {
                        if let Ok(subscription) = Subscription::try_from(subscription) {
                            if let Some(object) = inner.objects.get(&subscription.port_name) {
                                object(subscription.value.clone());
                            }
                        }
                    } else {
                        inner.message_queue.push(response);
                    }
                }
            }

            {
                let mut inner = lock(&inner_ft_swarm).await;

                // Handle outputs
                if let Some(data) = inner.write_queue.pop() {
                    serial_port.write_line(data).expect("Write line failure");
                }
            }

            sleep(Duration::from_millis(15)).await;
        }
    }


pub(crate) async fn push_cache(&self, object: Box<dyn Fn(RPCReturnParam) + Send>, name: &str) {
    let mut inner = lock(&self.inner).await;
    inner.objects.insert(name.to_string(), object);
}

/// Low-level method to send a command to the ftSwarm. Only use this as a last resort
pub async fn send_command(&self, command: FtSwarmCommand) {
    let mut inner = lock(&self.inner).await;
    inner.write_queue.push(command);
}

/// Low-level method to receive a response to the ftSwarm. Only use this as a last resort
pub async fn read_response(&self) -> Result<RPCReturnParam, String> {
    let (handle, mut recv) = SenderHandle::create();
    {
        let mut inner = lock(&self.inner).await;
        inner.message_queue.push_sender(&handle);
    }

    let response = recv.recv().await.unwrap();

    {
        let mut inner = lock(&self.inner).await;
        inner.message_queue.drop_sender(&handle);
    }

    match response {
        S2RMessage::RPCResponse(data) => Ok(RPCReturnParam::from(data)),
        S2RMessage::Error(data) => Err(data),
        any => Err(format!("Received non-RPCResponse message, {:?}", any).to_string()),
    }
}


/// Low-level method to send a command to the ftSwarm and receive a response. Only use this as a last resort
pub async fn transact(&self, command: FtSwarmCommand) -> Result<RPCReturnParam, String> {
    // Subscribe commands don't return a response
    let is_subscription = match &command {
        FtSwarmCommand::RPC(cmd) => cmd.function == RpcFunction::Subscribe,
        _ => false,
    };

    self.send_command(command).await;

    if is_subscription {
        return Ok(RPCReturnParam::Ok);
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
pub async fn halt(&self) {
    self.send_command(FtSwarmCommand::Direct(FtSwarmDirectCommand::Halt)).await;
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