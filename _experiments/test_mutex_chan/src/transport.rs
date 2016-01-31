use std::sync::{Arc, Mutex};
use query::Query;
use receiver::Receiver;
use outvalue::Outvalue;

pub struct Transport<T, R> {
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Mutex<Outvalue<R>>>,
    pub transform : Box<Fn(T) -> R>,
}
    
pub trait TransportIn<T> {
    fn send(self, T);       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

pub trait TransportOut<R> {
    fn ready(self);
}

//TODO - dodać implementacja TransportOut dla Receiver

impl<R, T> TransportOut<R> for Transport<T, R> {
    fn ready(self) {
    }
}