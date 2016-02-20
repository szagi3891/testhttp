use std::sync::{Arc, Mutex, Condvar};
use std::collections::linked_list::LinkedList;

use transport::TransportOut;


enum GetResult<R> {
    List(LinkedList<Box<TransportOut<R> + Send>>),
    Value(R)
}


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

        match self.get_value() {
            
            GetResult::Value(value) => {
                return value;
            },
            
            GetResult::List(mut list_invitation) => {
                
                loop {

                    match list_invitation.pop_back() {

                        Some(invit_item) => {
                            invit_item.ready();
                        },
                        
                        None => {
                            return self.get_sync();
                        }
                    }
                }
            }
        }
    }
    
    fn get_value(&self) -> GetResult<R> {

        let mut guard = self.mutex.lock().unwrap();

        let value = guard.take();

        match value {

            Some(value) => {
                return GetResult::Value(value);
            }
            None => {},
        }

        GetResult::List(guard.get_list_and_drain())
    }
    
    
    fn get_sync(&self) -> R {

        let mut guard = self.mutex.lock().unwrap();

        loop {
            
            let value = guard.take();

            match value {

                Some(value) => {
                    return value;
                }

                None => {

                    //println!("dalej pusta wartość w schowku, czekam dalej");
                }
            }

            guard = self.cond.wait(guard).unwrap();
        }
    }
}



//TODO zrobić te pola ukryte

pub struct OutvalueInner<R> {
    pub end_flag : bool,
    pub value : Option<R>,
    pub list  : LinkedList<Box<TransportOut<R> + Send>>,
}

impl<R> OutvalueInner<R> {
    
    
    fn new() -> Mutex<OutvalueInner<R>> {
        
        Mutex::new(OutvalueInner{
            end_flag : false,
            value : None,
            list  : LinkedList::new(),
        })
    }
    
    
    fn take(&mut self) -> Option<R> {
        self.value.take()
    }
    
    
    fn get_list_and_drain(&mut self) -> LinkedList<Box<TransportOut<R> + Send>> {


        let mut out = LinkedList::new();

        loop {
            match self.list.pop_front() {
                Some(item) => out.push_back(item),
                None => return out
            }
        }
    }
}
