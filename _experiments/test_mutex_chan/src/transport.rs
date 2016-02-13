use std::sync::{Arc, Mutex};
use query::Query;
use outvalue::Outvalue;
use fnconvert::{Fnconvert, Convert};


pub trait TransportIn<T> {
    fn send(self : Box<Self>, Box<T>) -> Option<Box<T>>;       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

pub trait TransportOut<R> {
    fn ready(self : Box<Self>);
}


//TODO - te wartości powinny się stać prywatne doceloo

pub struct Transport<T, R> {
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Outvalue<R>>,
    //pub fnconvert : Fnconvert<T,R>,
    pub fnconvert : Box<Convert<T,R>>,
}


impl<T, R> TransportIn<T> for Transport<T, R> 
    where
        T : Send + Sync + Clone + 'static ,
        R : Send + Sync + Clone + 'static {
    
    
    fn send(self: Box<Self>, value: Box<T>) -> Option<Box<T>> {
        
                    //TODO - to trzeba jakoś wyprostować - to klonowanie jest głupie
        //let mut outvalue_guard = self.outvalue.mutex.lock().unwrap();
        
        
        let outvalue = self.outvalue.clone();
    
        let mut outvalue_guard = outvalue.mutex.lock().unwrap();
        
        
        if outvalue_guard.end_flag {
                                            //pozwalamy na usunięcie obiektu transportu gdyż jego czas minął
            return Some(value);
        
        }
        
        
        
                                        //wysyłanie, może się nie udać, wtedy zwracamy originalną wartość
        if outvalue_guard.value.is_some() {

            outvalue_guard.list.push_back(self);
            
            Some(value)

        } else {

            outvalue_guard.value = Some(self.fnconvert.conv(value));
                                    //powiadom wszystkie uśpione wątki że podano do stołu
            self.outvalue.cond.notify_all();

            outvalue_guard.list.push_back(self);

            None 
        }
    }
}


impl<T, R> TransportOut<R> for Transport<T, R>
    where
        T : Send + Sync + Clone + 'static ,
        R : Send + Sync + Clone + 'static {
    
    fn ready(self: Box<Self>) {
        
                    //TODO - to trzeba jakoś wyprostować - to klonowanie jest głupie
        
        let query = self.query.clone();
        
        let mut query_guard = query.lock().unwrap();
        
        query_guard.senders.push_back(self);
        
        query_guard.sending();
    }
}

