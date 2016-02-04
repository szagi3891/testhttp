use std::sync::{Arc, Mutex, Condvar};
use std::collections::linked_list::LinkedList;

use transport::TransportOut;


//TODO - te właściwości trzeba uprywatnić, dostęp do stanu ma się odbywać wyłącznie poprzez dedykowane metody


pub struct Outvalue<R> {
    pub mutex : Mutex<OutvalueInner<R>>,
    pub cond  : Condvar,
}

impl<R> Outvalue<R> {
    
    pub fn new() -> Arc<Outvalue<R>> {
        
        Arc::new(Outvalue {
            mutex : OutvalueInner::new(),
            cond  : Condvar::new(),
        })
    }
    
    pub fn get(&self) -> R {

        let mut guard = self.mutex.lock().unwrap();

        loop {
            
            let value = guard.value.take();

            match value {

                Some(value) => {
                    return value;
                }

                None => {

                    println!("dalej pusta wartość w schowku, czekam dalej");
                }
            }

            guard = self.cond.wait(guard).unwrap();
        }
    }
}


pub struct OutvalueInner<R> {
    pub value : Option<R>,
    pub list  : LinkedList<Box<TransportOut<R> + Send>>,
}

impl<R> OutvalueInner<R> {
    
    fn new() -> Mutex<OutvalueInner<R>> {
        
        Mutex::new(OutvalueInner{
            value : None,
            list  : LinkedList::new(),
        })
    }
}
