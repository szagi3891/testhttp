use std::sync::Arc;
use transport::TransportOut;
use outvalue::{Outvalue, OutvalueInner};
use std::collections::linked_list::LinkedList;
use std::sync::MutexGuard;


pub struct Receiver<R> {
    pub outvalue : Arc<Outvalue<R>>,
}


impl<R> Receiver<R> {
    
    pub fn new(outvalue: Arc<Outvalue<R>>) -> Receiver<R> {
        Receiver{
            outvalue : outvalue
        }
    }
    
    pub fn get(&self) -> R {
        
        //let mut list_or_value = self.outvalue.get_value_or_invitation();
        
        let mut list_invitation : LinkedList<Box<TransportOut<R> + Send>> = {
            
            let mut guard = self.outvalue.mutex.lock().unwrap();
            
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
                    return self.outvalue.get();
                }
            }
        }
    }
    
    
    fn get_list_copy(&self, guard: &mut MutexGuard<OutvalueInner<R>>) -> LinkedList<Box<TransportOut<R> + Send>> {


        let mut out = LinkedList::new();

        loop {
            match guard.list.pop_front() {
                Some(item) => out.push_back(item),
                None => return out
            }
        }
    }
}
