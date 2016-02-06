use std::sync::{Arc, Mutex};
use query::Query;
use outvalue::Outvalue;


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
    pub transform : Box<Fn(T) -> R + Send>,
}

impl<T:Send+Clone+'static, R:Send+Clone+'static> Transport<T, R> {
    
    fn transform_value(&self, value: Box<T>) -> R {
        
                    //TODO - potrzebny jest lepszy sposób na wywołanie clousera zapisanego w zmiennej struktury
                
        match self.transform {

            ref transform => {

                transform((*value).clone())
            }
        }       
    }
}

impl<T:Send+Clone+'static, R:Send+Clone+'static> TransportIn<T> for Transport<T, R> {
    
    
    fn send(self: Box<Self>, value: Box<T>) -> Option<Box<T>> {
        
                    //TODO - to trzeba jakoś wyprostować - to klonowanie jest głupie
        //let mut outvalue_guard = self.outvalue.mutex.lock().unwrap();
        
        
        let outvalue = self.outvalue.clone();
    
        let mut outvalue_guard = outvalue.mutex.lock().unwrap();
        
        
        //jeśli transport na nową lokalizację to
        //w tym miejscu się trzeba wstrzelić z przepięciem
        
        
        
                                        //wysyłanie, może się nie udać, wtedy zwracamy originalną wartość
        if outvalue_guard.value.is_some() {

            outvalue_guard.list.push_back(self);

            Some(value)

        } else {

            outvalue_guard.value = Some(self.transform_value(value));
                                    //powiadom wszystkie uśpione wątki że podano do stołu
            self.outvalue.cond.notify_all();

            outvalue_guard.list.push_back(self);

            None 
        }
    }
}


impl<T:Send+Clone+'static, R:Send+Clone+'static> TransportOut<R> for Transport<T, R> {
    
    fn ready(self: Box<Self>) {
        
                    //TODO - to trzeba jakoś wyprostować - to klonowanie jest głupie
        
        let query = self.query.clone();
        
        let mut query_guard = query.lock().unwrap();
        
        query_guard.senders.push_back(self);
        
        query_guard.sending();
    }
}



