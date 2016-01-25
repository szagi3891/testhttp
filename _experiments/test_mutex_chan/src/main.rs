use std::sync::{Arc, Mutex, Condvar};
use std::collections::LinkedList;

fn chan() {
}

struct Sender<T> {
    query : Arc<Mutex<StateQuery<T>>>,
}

struct StateQuery<T> {
    //list : LinkedList<Box<TransportIn<T>>>,
    list : Vec<Box<TransportIn<T>>>,
}


struct Transport<T,R> {
    query    : Arc<Mutex<StateQuery<T>>>,
    receiver : Arc<Mutex<Receiver<R>>>,
}

trait TransportIn<T> {      //T:Sized
    fn send(self, T);       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

trait TransportOut<R> {
    fn ready(self);
}

struct Receiver<R> {
    mutex : Mutex<ReceiverInner<R>>,
    cond  : Condvar,
}

struct ReceiverInner<R> {
    list  : Vec<Box<TransportOut<R>>>,
}

impl<R> Receiver<R> {
    
    fn new() -> Arc<Receiver<R>> {
        Arc::new(Receiver{
            mutex : Mutex::new(ReceiverInner::new()),
            cond  : Condvar::new(),
        })
    }
    
    /*
    fn save() {
    }
    
    fn get() -> R {   
    }
    */
}

impl<R> ReceiverInner<R> {
    fn new() -> ReceiverInner<R> {
        
        ReceiverInner{
            list  : Vec::new(),
        }
    }
}

//Sender
//StateQuery
//Transport
//Receiver


fn main() {
    
    println!("test ... zx");
}