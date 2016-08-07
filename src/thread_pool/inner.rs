use std::boxed::FnBox;
use std::sync::mpsc::{channel, Sender};
use std::collections::HashMap;
use task_async::{Task};
use std::thread;

use thread_pool::types::{ParamTrait, RespTrait, CounterType, WorkerBuilderType, WorkerFunctionType};
use thread_pool::sender_id::{SenderId};

pub struct Inner<Param: ParamTrait, Resp: RespTrait> {
    builder      : WorkerBuilderType<Param>,
    task         : HashMap<Param, Task<Resp>>,                      //TODO - zamienić na kolejkę, lub dodać jakieś kolejkowanie
                                                                    //aby czas oczekiwania był przewidywalny
    workers_busy : HashMap<CounterType  , SenderId<Param>>,
    workers_idle : HashMap<CounterType  , SenderId<Param>>,
}

impl<Param, Resp> Inner<Param, Resp> where Param: ParamTrait, Resp: RespTrait {
        
    pub fn new(count: CounterType, workerBuilder: WorkerBuilderType<Param>) -> Inner<Param, Resp> {

        let mut inst = Inner {
            builder      : workerBuilder,
            task         : HashMap::new(),
            workers_busy : HashMap::new(),
            workers_idle : HashMap::new(),
        };

        for i in 0..count {
            let workerFunction = (inst.builder)();
            inst.create_worker(i, workerFunction);
        }

        inst
    }
    
    fn create_worker(&mut self, id: CounterType, workerFunction: WorkerFunctionType<Param>) {
        let (sender, receiver) = channel();

        let sender_id = SenderId::new(id, sender);

        self.move_to_idle(sender_id);
                //thread::spawn(move || {
    }
    
    fn move_to_idle(&mut self, sender_id: SenderId<Param>) {
        
    }
}
