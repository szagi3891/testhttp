use std::sync::mpsc::Sender;

use thread_pool::types::{CounterType};

pub struct SenderId<Param> {
    id: CounterType,
    sender: Sender<Param>
}

impl<Param> SenderId<Param> where Param: Send + Sync + 'static {
    pub fn new(id: CounterType, sender: Sender<Param>) -> SenderId<Param> {
        SenderId {
            id: id,
            sender: sender
        }
    }
    
    pub fn id(&self) -> CounterType {
        self.id.clone()
    }
    
    pub fn send(&self, value: Param) {
        self.sender.send(value).unwrap();
    }
}
