use std::sync::{Arc, Mutex};
use query::Query;
use receiver::Receiver;
use outvalue::Outvalue;


pub trait TransportIn<T> {
    fn send_test(&self);
    fn send(self, Box<T>);       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

pub trait TransportOut<R> {
    fn ready(self);
}



pub struct Transport<T, R> {
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Mutex<Outvalue<R>>>,
    pub transform : Box<Fn(T) -> R>,
}
    


impl<T, R> TransportIn<T> for Transport<T, R> {
    
    fn send_test(&self) {
        println!("testowe wysyłanie");
    }
        
    fn send(self, value: Box<T>) {
        
        println!("wysyłam transportem wartość");
    }
}


impl<T, R> TransportOut<R> for Transport<T, R> {
    
    fn ready(self) {
    }
}



