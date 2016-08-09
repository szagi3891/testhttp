use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel};
use std::collections::HashMap;
use std::thread;
use std::collections::VecDeque;

use thread_pool::types::{CounterType, WorkerFunctionType};
use thread_pool::sender_id::{SenderId};
use thread_pool::receiver_id::{ReceiverId};
use thread_pool::autoid::{AutoId};

pub struct Inner<Param: Send + Sync + 'static> {
    autoid       : AutoId,
    task         : VecDeque<Param>,
    workers_idle : VecDeque<SenderId<Param>>,
    workers_busy : HashMap<CounterType, SenderId<Param>>,
}

impl<Param> Inner<Param> where Param: Send + Sync + 'static {
        
    pub fn new() -> Inner<Param> {
        Inner {
            autoid       : AutoId::new(),
            task         : VecDeque::new(),
            workers_idle : VecDeque::new(),
            workers_busy : HashMap::new(),
        }
    }

    pub fn create_worker(&mut self,
        inner: Arc<Mutex<Inner<Param>>>,
        worker_function: WorkerFunctionType<Param>) {

        let (sender, receiver) = channel();

        let id = self.autoid.get();

        let sender_id = SenderId::new(id.clone(), sender);
        self.workers_idle.push_back(sender_id);

        let receiver_id = ReceiverId::new(id, receiver);

        thread::spawn(move || {

            loop {
                match receiver_id.recv() {
                    Some(param) => {

                        (worker_function)(param);

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
                self.workers_idle.push_back(sender_id);
                self.refresh_state();
            },
            None => panic!("Niespodziewane odgałężienie"),
        }
    }
    
    fn set_as_busy(&mut self, sender_id: SenderId<Param>) {
        
        let id = sender_id.id();
        let result = self.workers_busy.insert(id, sender_id);

        match result {
            Some(_) => panic!("Niespodziewane odgałężienie"),
            None => {
                return;
            }
        }
    }

    pub fn run(&mut self, param: Param) {
        
        if self.task.len() == 0 {
            
            match self.workers_idle.pop_front() {

                Some(worker_sender) => {
                    worker_sender.send(param);
                    self.set_as_busy(worker_sender);
                },
                None => {
                    self.task.push_back(param);
                },
            }

        } else {
            self.task.push_back(param);
        }
    }
    
    pub fn refresh_state(&mut self) {
        loop {
            match (self.workers_idle.pop_front(), self.task.pop_front()) {
                
                (Some(worker_sender), Some(param)) => {
                    worker_sender.send(param);
                    self.set_as_busy(worker_sender);
                },
                
                (Some(worker_sender), None) => {
                    self.workers_idle.push_front(worker_sender);
                    return;
                },
                
                (None, Some(param)) => {
                    self.task.push_front(param);
                    return;
                },
                (None, None) => {
                    return;
                }
            }
        }
    }
}
