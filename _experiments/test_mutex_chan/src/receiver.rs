use std::sync::Arc;
use transport::TransportOut;
use outvalue::{Outvalue, GetResult};


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
        
        match self.outvalue.get() {
            
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
                            return self.outvalue.get_sync();
                        }
                    }
                }
            }
        }   
    }
}
