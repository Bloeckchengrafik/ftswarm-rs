use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::message_queue::ReturnQueue;
use crate::serial::{SerialCommunication, SwarmSerialPort};
use crate::swarm_object::SwarmObject;

mod message_queue;
pub mod swarm_object;
mod serial;

struct InnerFtSwarm {
    objects: HashMap<String, Box<dyn SwarmObject + Send>>,
    message_queue: ReturnQueue,
}

impl InnerFtSwarm {
    fn new() -> Self {
        InnerFtSwarm {
            objects: HashMap::new(),
            message_queue: ReturnQueue::new(),
        }
    }
}

pub struct FtSwarm<Serial : SwarmSerialPort + 'static> {
    inner: Arc<Mutex<InnerFtSwarm>>,
    _serial: PhantomData<Serial>
}

impl <Serial : SwarmSerialPort + 'static> FtSwarm<Serial> {
    pub fn new(serial: Serial) -> Self {
        let inner = Arc::new(Mutex::new(InnerFtSwarm::new()));

        let inner_for_thread = inner.clone();
        tokio::spawn(async move {
            FtSwarm::input_loop(inner_for_thread, serial).await;
        });

        FtSwarm {
            inner,
            _serial: PhantomData
        }
    }

    async fn input_loop(inner_ft_swarm: Arc<Mutex<InnerFtSwarm>>, serial_port: Serial) {
        loop {

        }
    }
}

impl Default for FtSwarm<SerialCommunication> {
    fn default() -> Self {
        FtSwarm::new(SerialCommunication::default())
    }
}