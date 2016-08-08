use std::sync::{Arc, Mutex};

mod sender_id;
mod receiver_id;
mod types;
mod inner;
mod autoid;

use thread_pool::types::{ParamTrait, CounterType, WorkerBuilderType};
use thread_pool::inner::{Inner};

#[derive(Clone)]
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

    pub fn run(&self, param: Param) {
        let mut guard = self.inner.lock().unwrap();
        guard.run(param);
    }
}