use std::sync::{Arc, Mutex, Condvar};
use transport::TransportOut;
use outvalue::Outvalue;
use std::collections::linked_list::LinkedList;


pub struct Receiver<R> {
    pub mutex : Arc<Mutex<Outvalue<R>>>,
    cond  : Condvar,
}


impl<R> Receiver<R> {
    
    pub fn new(outvalue: Arc<Mutex<Outvalue<R>>>) -> Receiver<R> {
        Receiver{
            mutex : outvalue,
            cond  : Condvar::new(),
        }
    }
    
    pub fn get(&mut self) -> R {
        
        let mut list_invitation : LinkedList<Box<TransportOut<R> + Send>> = {
            
            let mut guard = self.mutex.lock().unwrap();
            
            let value = guard.value.take();
            
            match value {
                
                Some(value) => {
                    return value;
                }
               None => {},
            }
            
            get_list_copy(guard)
        };
        
        //roześlij zaproszenia do nadawców
            
        println!("dasd {}", list_invitation.len());
        
        loop {
            
            match list_invitation.pop() {
                Some(invit_item) => {
                    invit_item.ready();
                },
                None => {
                    return self.get_in_loop();
                }
            }
        }
    }
    
    fn get_list_copy() {
        

        let mut out = LinkedList::new();

        loop {
            match guard.list.pop_front() {
                Some(item) => out.back_push(item),
                None => 
            }
        }


        guard.list.drain(..).collect()
    }
        
    fn get_in_loop(&mut self) -> R {
        
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

           
/*
impl<R> Clone for Receiver<R> {
    
    fn clone(&self) -> Self {
    
    }

    fn clone_from(&mut self, source: &Self) { ... }
}
*/