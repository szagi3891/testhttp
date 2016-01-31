use std::sync::{Arc, Mutex};
use query::Query;
use receiver::Receiver;
use outvalue::Outvalue;


pub trait TransportIn<T> : Sized {
    fn send(self, T);       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

pub trait TransportOut<R> : Sized {
    fn ready(self);
}



pub struct Transport<T, R> {
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Mutex<Outvalue<R>>>,
    pub transform : Box<Fn(T) -> R>,
}
    


impl<T, R> TransportIn<T> for Transport<T, R> {
    
    fn send(self, value: T) {
        
        println!("wysyłam transportem wartość");
    }
}


impl<T, R> TransportOut<R> for Transport<T, R> {
    
    fn ready(self) {
    }
}


