use std::sync::{Arc, Mutex};

mod sender_id;
mod types;
mod inner;

use thread_pool::types::{ParamTrait, RespTrait, CounterType, FunctionWorker};
use thread_pool::inner::{Inner};

pub struct ThreadPool<Param: ParamTrait, Resp: RespTrait> {
    inner: Arc<Mutex<Inner<Param, Resp>>>,
}

impl<Param, Resp> ThreadPool<Param, Resp> where 
    Param: ParamTrait ,
    Resp : RespTrait {
        
    pub fn new(count: CounterType, fnWork: FunctionWorker<Param>) -> ThreadPool<Param, Resp> {
        ThreadPool {
            inner: Arc::new(Mutex::new(Inner::new(count, fnWork)))
        }
    }
}
