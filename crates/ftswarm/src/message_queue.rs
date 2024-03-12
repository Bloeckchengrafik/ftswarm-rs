use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use ftswarm_proto::command::FtSwarmCommand;
use ftswarm_proto::message_parser::S2RMessage;
use ftswarm_proto::Serialized;

type Id = i128;

pub struct ReturnQueue {
    queue: Vec<S2RMessage>,
    senders: HashMap<Id, Arc<mpsc::Sender<S2RMessage>>>,
}

pub struct SenderHandle {
    pub receiver: mpsc::Sender<S2RMessage>,
    uid: Id,
}

impl SenderHandle {
    pub fn new(receiver: mpsc::Sender<S2RMessage>) -> Self {
        SenderHandle {
            receiver,
            uid: rand::random::<Id>() << 64 | std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros() as Id,
        }
    }

    pub fn create() -> (Self, mpsc::Receiver<S2RMessage>) {
        let (s, r) = mpsc::channel(1);
        (SenderHandle::new(s), r)
    }
}

impl ReturnQueue {
    pub fn new() -> Self {
        ReturnQueue {
            queue: Vec::new(),
            senders: HashMap::new(),
        }
    }

    pub fn push(&mut self, value: S2RMessage) {
        if let S2RMessage::Log(val) = value {
            log::debug!("{}", val);
            return;
        }

        self.queue.push(value.clone());
        for (_, func) in self.senders.iter() {
            let fnc = func.clone();
            let _ = fnc.try_send(value.clone());
        }
    }

    pub fn push_sender(&mut self, sender: &SenderHandle) {
        self.senders.insert(sender.uid, Arc::new(sender.receiver.clone()));
    }

    pub fn drop_sender(&mut self, handle: &SenderHandle) {
        self.senders.remove(&handle.uid);
    }
}

pub struct WriteQueue {
    queue: Vec<FtSwarmCommand>,
}

impl WriteQueue {
    pub fn new() -> Self {
        WriteQueue {
            queue: Vec::new(),
        }
    }

    pub fn push(&mut self, value: FtSwarmCommand) {
        self.queue.push(value);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.queue.pop().map(|value| value.serialize())
    }
}