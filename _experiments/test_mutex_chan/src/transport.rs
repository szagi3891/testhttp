use std::sync::{Arc, Mutex};
use query::Query;
use receiver::Receiver;
use outvalue::Outvalue;


pub trait TransportIn<T> {
    fn send(self : Box<Self>, Box<T>) -> Option<Box<T>>;       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

pub trait TransportOut<R> {
    fn ready(self : Box<Self>);
}



pub struct Transport<T, R> {
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Mutex<Outvalue<R>>>,
    pub transform : Box<Fn(T) -> R + Send>,
}


impl<T:Send+Clone+'static, R:Send+Clone+'static> TransportIn<T> for Transport<T, R> {
    
    fn send(self: Box<Self>, value: Box<T>) -> Option<Box<T>> {
        
        let outvalue = self.outvalue.clone();
        
        let mut outvalue_guard = outvalue.lock().unwrap();
        
                                        //wysyłanie, może się nie udać, wtedy zwracamy originalną wartość
        let out_value = {
            
            if outvalue_guard.value.is_some() {
                
                Some(value)
            
            } else {
                            //TODO - potrzebny jest lepszy sposób na wywołanie clousera zapisanego w zmiennej struktury
                
                let new_value = match self.transform {
                    
                    ref transform => {
                        
                        transform((*value).clone())
                    }
                };
                
                outvalue_guard.value = Some(new_value);
                
                None
            }
        };
        
        outvalue_guard.list.push_back(self);
        
        out_value
    }
}


impl<T:Send+Clone+'static, R:Send+Clone+'static> TransportOut<R> for Transport<T, R> {
    
    fn ready(self: Box<Self>) {
        
        let query = self.query.clone();
        
        let mut query_guard = query.lock().unwrap();
        
        query_guard.senders.push_back(self);
        
        query_guard.sending();
    }
}



