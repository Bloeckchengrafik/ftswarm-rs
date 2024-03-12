use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::mpsc;
use ftswarm_proto::command::FtSwarmCommand;
use ftswarm_proto::message_parser::ReturnMessageType;
use ftswarm_proto::Serialized;

type Id = usize;

pub struct ReturnQueue {
    queue: Vec<ReturnMessageType>,
    funcs: HashMap<Id, Box<dyn Fn(&ReturnMessageType) + Send + 'static>>,
}

impl ReturnQueue {
    pub fn new() -> Self {
        ReturnQueue {
            queue: Vec::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn push(&mut self, value: ReturnMessageType) {
        if let ReturnMessageType::Log(val) = value {
            log::debug!("{}", val);
            return;
        }

        self.queue.push(value.clone());
        for (_, func) in self.funcs.iter() {
            let fnc = func.clone();
            fnc(&value);
        }
    }

    pub fn take_subscription(&mut self) -> Option<ReturnMessageType> {
        let mut index = None;
        for (i, value) in self.queue.iter().enumerate() {
            if let ReturnMessageType::Subscription(_) = value {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            return Some(self.queue.remove(i));
        }

        None
    }

    pub fn take_rpc_response_or_error(&mut self) -> Option<ReturnMessageType> {
        let mut index = None;
        for (i, value) in self.queue.iter().enumerate() {
            if let ReturnMessageType::RPCResponse(_) = value {
                index = Some(i);
                break;
            }
            if let ReturnMessageType::Error(_) = value {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            return Some(self.queue.remove(i));
        }

        None
    }

    async fn take_or_wait(&mut self, filter: Box<dyn Fn(&ReturnMessageType) -> bool + Send + 'static>) -> ReturnMessageType {
        let (s, mut r) = mpsc::channel(1);
        let uid = rand::random::<Id>();
        let sender = Arc::new(s);
        let sender_for_closure = sender.clone();
        self.funcs.insert(uid, Box::new(move |value: &ReturnMessageType| {
            if filter(value) {
                let s = sender_for_closure.clone();
                let _ = s.send(value.clone());
            }
        }));

        let _ = r.recv().await.unwrap();
        // Pop the function we just added
        self.funcs.remove(&uid);
        self.queue.pop().unwrap()
    }

    async fn take_or_wait_subscription(&mut self) -> ReturnMessageType {
        self.take_or_wait(Box::new(|value| {
            if let ReturnMessageType::Subscription(_) = value {
                return true;
            }
            false
        })).await
    }

    async fn take_or_wait_rpc_response_or_error(&mut self) -> ReturnMessageType {
        self.take_or_wait(Box::new(|value| {
            if let ReturnMessageType::RPCResponse(_) = value {
                return true;
            }
            if let ReturnMessageType::Error(_) = value {
                return true;
            }
            false
        })).await
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