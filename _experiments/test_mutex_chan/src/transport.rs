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
    pub transform : Box<Fn(T) -> R + 'static + Send + Sync>,
}

impl<T, R> Transport<T, R>
    where
        T : Send + Clone + 'static ,
        R : Send + Clone + 'static {
    
    fn transform_value(&self, value: Box<T>) -> R {
        
                    //TODO - potrzebny jest lepszy sposób na wywołanie clousera zapisanego w zmiennej struktury
                
        match self.transform {

            ref transform => {

                transform((*value).clone())
            }
        }
    }
}

impl<T, R> TransportIn<T> for Transport<T, R> 
    where
        T : Send + Clone + 'static ,
        R : Send + Clone + 'static {
    
    
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

            outvalue_guard.value = Some(self.transform_value(value));
                                    //powiadom wszystkie uśpione wątki że podano do stołu
            self.outvalue.cond.notify_all();

            outvalue_guard.list.push_back(self);

            None 
        }
    }
}


impl<T, R> TransportOut<R> for Transport<T, R>
    where
        T : Send + Clone + 'static ,
        R : Send + Clone + 'static {
    
    fn ready(self: Box<Self>) {
        
                    //TODO - to trzeba jakoś wyprostować - to klonowanie jest głupie
        
        let query = self.query.clone();
        
        let mut query_guard = query.lock().unwrap();
        
        query_guard.senders.push_back(self);
        
        query_guard.sending();
    }
}



