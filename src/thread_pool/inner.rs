use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::collections::HashMap;
use task_async::{Task};
use std::thread;
use std::collections::VecDeque;

use thread_pool::{ThreadPool};
use thread_pool::types::{ParamTrait, CounterType, WorkerFunctionType};
use thread_pool::sender_id::{SenderId};
use thread_pool::receiver_id::{ReceiverId};
use thread_pool::autoid::{AutoId};

pub struct Inner<Param: ParamTrait> {
    autoid       : AutoId,
    task         : VecDeque<(Param)>,
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

impl<Param> Inner<Param> where Param: ParamTrait {
        
    pub fn new() -> Inner<Param> {
        Inner {
            autoid       : AutoId::new(),
            task         : VecDeque::new(),
            workers_busy : HashMap::new(),
            workers_idle : HashMap::new(),
        }
    }

    pub fn create_worker(&mut self,
        inner: Arc<Mutex<Inner<Param>>>,
        workerFunction: WorkerFunctionType<Param>) {

        let (sender, receiver) = channel();

        let id = self.autoid.get();

        let sender_id = SenderId::new(id.clone(), sender);
        self.workers_idle.insert(id.clone(), sender_id);

        let receiver_id = ReceiverId::new(id, receiver);

        thread::spawn(move || {

            loop {
                match receiver_id.recv() {
                    Some(param) => {

                        (workerFunction)(param);

                        let mut guard = inner.lock().unwrap();
                        guard.set_as_idle(receiver_id.id());
                    },
                    None => {
                        return;
                    }
                }
            }
        });
    }

    fn set_as_idle(&mut self, id: CounterType) {
        
        let value = self.workers_busy.remove(&id);
        
        match value {
            Some(sender_id) => {
                
                let result = self.workers_idle.insert(id, sender_id);
                
                match result {
                    Some(_) => panic!("Niespodziewane odgałężienie"),
                    None => {
                        return;
                    }
                }
            },
            None => panic!("Niespodziewane odgałężienie"),
        }
    }
}
