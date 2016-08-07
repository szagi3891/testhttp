use std::sync::{Arc, Mutex};

mod sender_id;
mod receiver_id;
mod types;
mod inner;
mod autoid;

use thread_pool::types::{ParamTrait, CounterType, WorkerBuilderType};
use thread_pool::inner::{Inner};

pub struct ThreadPool<Param: ParamTrait> {
    inner: Arc<Mutex<Inner<Param>>>,
}

impl<Param> ThreadPool<Param> where Param: ParamTrait {
        
    pub fn new(count: CounterType, workerBuilder: WorkerBuilderType<Param>) -> ThreadPool<Param> {

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
            guard.create_worker(self_clone.inner.clone(), workerFunction);
        }
    }

    /* pub fn run(param: Param) {
        //...
    } */
}

    //chociaż chyba lepiej, żeby pula wątków przyjmowała zwykłego callbacka
        
    //stworzyć typ Tast który będzie trait
    //ten obiekt będzie miał jedną wymaganą metodę .response(RespTrait)

    //właściwy moduł Task-ów, będzie dla swojego taska, implementował trait tego wyżej

//TODO - zastąpić #[derive]

impl<Param> Clone for ThreadPool<Param> where Param: ParamTrait  {
    
    fn clone(&self) -> ThreadPool<Param> {
        ThreadPool {
            inner : self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &ThreadPool<Param>) {
        self.inner = source.inner.clone();
    }
}