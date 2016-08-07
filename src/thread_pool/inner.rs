use std::sync::mpsc::{channel, Sender};
use std::collections::HashMap;
use task_async::{Task};
use std::thread;

use thread_pool::types::{ParamTrait, RespTrait, CounterType};
use thread_pool::sender_id::{SenderId};

pub struct Inner<Param: ParamTrait, Resp: RespTrait> {
    task         : HashMap<Param, Task<Resp>>,
    workers_busy : HashMap<CounterType  , SenderId<Param>>,
    workers_idle : HashMap<CounterType  , SenderId<Param>>,
}

impl<Param, Resp> Inner<Param, Resp> where Param: ParamTrait, Resp: RespTrait {
        
    pub fn new(count: CounterType) -> Inner<Param, Resp> {

        let mut inst = Inner {
            task         : HashMap::new(),
            workers_busy : HashMap::new(),
            workers_idle : HashMap::new(),
        };

        for i in 0..count {
            inst.create_worker(i);
        }

        inst
    }
    
    fn create_worker(&mut self, id: CounterType) {
        let (sender, receiver) = channel();

        let sender_id = SenderId::new(id, sender);

        self.move_to_idle(sender_id);
                //thread::spawn(move || {
    }
    
    fn move_to_idle(&mut self, sender_id: SenderId<Param>) {
        
    }
}
