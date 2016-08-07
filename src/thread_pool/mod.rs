use std::sync::{Arc, Mutex};

mod sender_id;
mod receiver_id;
mod types;
mod inner;
mod autoid;

use thread_pool::types::{ParamTrait, RespTrait, CounterType, WorkerBuilderType};
use thread_pool::inner::{Inner};

pub struct ThreadPool<Param: ParamTrait, Resp: RespTrait> {
    inner: Arc<Mutex<Inner<Param, Resp>>>,
}

impl<Param, Resp> ThreadPool<Param, Resp> where 
    Param: ParamTrait ,
    Resp : RespTrait {
        
    pub fn new(count: CounterType, workerBuilder: WorkerBuilderType<Param>) -> ThreadPool<Param, Resp> {

        let inner = Inner::new();

        let pool = ThreadPool {
            inner: Arc::new(Mutex::new(inner))
        };
        
        pool.new_workers(count, workerBuilder);
        
        pool
    }

    fn new_workers(&self, count: CounterType, workerBuilder: WorkerBuilderType<Param>) {

        let self_clone = self.clone();
        let mut guard = self.inner.lock().unwrap();
        
        for i in 0..count {
            let workerFunction = (workerBuilder)();
            guard.create_worker(self_clone.clone(), workerFunction);
        }
    }
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //chociaż chyba lepiej, żeby pula wątków przyjmowała zwykłego callbacka
        
    //stworzyć typ Tast który będzie trait
        //ten obiekt będzie miał jedną wymaganą metodę .response(RespTrait)

    //właściwy moduł Task-ów, będzie dla swojego taska, implementował trait tego wyżej
/*
    pub fn run(param: Param, task: Task) {    
    }
*/
}

//TODO - zastąpić #[derive]

impl<Param, Resp> Clone for ThreadPool<Param, Resp> where 
    Param: ParamTrait ,
    Resp : RespTrait {
    
    fn clone(&self) -> ThreadPool<Param, Resp> {
        ThreadPool {
            inner : self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &ThreadPool<Param, Resp>) {
        self.inner = source.inner.clone();
    }
}