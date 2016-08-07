use std::sync::mpsc::{channel, Sender};
use std::collections::HashMap;
use task_async::{Task};
use std::thread;
use std::collections::VecDeque;

use thread_pool::{ThreadPool};
use thread_pool::types::{ParamTrait, RespTrait, CounterType, WorkerFunctionType};
use thread_pool::sender_id::{SenderId};
use thread_pool::receiver_id::{ReceiverId};
use thread_pool::autoid::{AutoId};

pub struct Inner<Param: ParamTrait, Resp: RespTrait> {
    autoid       : AutoId,
    task         : VecDeque<(Param, Task<Resp>)>,
    workers_busy : HashMap<CounterType  , SenderId<Param>>,
    workers_idle : HashMap<CounterType  , SenderId<Param>>,
}

/*
    pub fn push(&mut self, elem: T) {
        self.list.push_back(elem);
    }

    pub fn back_pop(&mut self, elem: T) {
        self.list.push_front(elem);
    }
*/

impl<Param, Resp> Inner<Param, Resp> where Param: ParamTrait, Resp: RespTrait {
        
    pub fn new() -> Inner<Param, Resp> {
        Inner {
            autoid       : AutoId::new(),
            task         : VecDeque::new(),
            workers_busy : HashMap::new(),
            workers_idle : HashMap::new(),
        }
    }

    pub fn create_worker(&mut self,
        thread_pool: ThreadPool<Param, Resp>,
        workerFunction: WorkerFunctionType<Param>) {

        let (sender, receiver) = channel();

        let id = self.autoid.get();
        let sender_id = SenderId::new(id.clone(), sender);
        let receiver_id = ReceiverId::new(id, receiver);
        
        self.move_to_idle(sender_id);

        thread::spawn(move || {
            loop {
                match receiver_id.recv() {
                    Some(param) => {
                        //let result =          //TODO - dodać wartość zwracaną przez tą funkcję
                        (workerFunction)(param);
                    },
                    None => {
                        return;
                    }
                }
            }
        });
    }
    
    fn move_to_idle(&mut self, sender_id: SenderId<Param>) {
        let id = sender_id.id();
        self.workers_idle.insert(id, sender_id);
    }
}
