use ftswarm_proto::message_parser::ReturnMessageType;

pub struct ReturnQueue {
    queue: Vec<ReturnMessageType>,
    watchers: Vec<Box<dyn FnMut(ReturnMessageType)>>
}

impl ReturnQueue {
    pub fn new() -> Self {
        ReturnQueue {
            queue: Vec::new(),
            watchers: Vec::new()
        }
    }

    pub fn push(&mut self, value: ReturnMessageType) {
        if let ReturnMessageType::Log(val) = value {
            log::debug!("{}", val);
            return;
        }

        self.queue.push(value);

        for watcher in self.watchers.iter_mut() {
            watcher(value.clone());
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

    async fn take_or_wait(&mut self, filter: Box<dyn Fn(&ReturnMessageType) -> bool>) -> ReturnMessageType {
        if let Some(value) = self.queue.iter().position(|value| filter(value)) {
            return self.queue.remove(value);
        }

        let (s, r) = async_channel::bounded(1);
        self.watchers.push(Box::new(move |value| {
            if filter(&value) {
                s.send_blocking(value).ok();
            }
        }));

        r.recv().await.unwrap()
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