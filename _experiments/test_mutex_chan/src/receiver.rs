use std::sync::{Arc, Mutex, Condvar};
use transport::TransportOut;
use outvalue::Outvalue;
use std::collections::linked_list::LinkedList;
use std::sync::MutexGuard;


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
            
            self.get_list_copy(&mut guard)
        };
        
        
                        //roześlij zaproszenia do nadawców
                        //ważne, do momentu wywołania get_in_loop nie możemy posiadać żadnego locka
        loop {
            
            match list_invitation.pop_back() {
                
                Some(invit_item) => {
                    invit_item.ready();
                },
                None => {
                    return self.get_in_loop();
                }
            }
        }
    }
    
    
    fn get_list_copy(&self, guard: &mut MutexGuard<Outvalue<R>>) -> LinkedList<Box<TransportOut<R> + Send>> {


        let mut out = LinkedList::new();

        loop {
            match guard.list.pop_front() {
                Some(item) => out.push_back(item),
                None => return out
            }
        }
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