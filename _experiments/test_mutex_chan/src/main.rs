use std::sync::{Arc, Mutex, Condvar};
use std::collections::LinkedList;

fn chan<T>() -> (Sender<T>, Arc<Receiver<T>>) {
    
    let query : Arc<Mutex<StateQuery<T>>> = StateQuery::new();
    let receiver : Arc<Receiver<T>>       = Receiver::new();
    let sender                            = Sender::new(query.clone());
    
    let transport : Transport<T, T> = Transport {
        query    : query.clone(),
        receiver : receiver.clone(),
        transform : createIdentity::<T>(),      //funkcja przejścia
    };
        
    {
        let mut inner = receiver.mutex.lock().unwrap();
        inner.list.push(Box::new(transport));
    }
    
    (sender, receiver)
}

struct Sender<T> {
    query : Arc<Mutex<StateQuery<T>>>,
}

impl<T> Sender<T> {
    
    fn new(query: Arc<Mutex<StateQuery<T>>>) -> Sender<T> {
        Sender {
            query : query
        }
    }
}

struct StateQuery<T> {
    //list : LinkedList<Box<TransportIn<T>>>,
    list : Vec<Box<TransportIn<T>>>,
}

impl<T> StateQuery<T> {
    fn new() -> Arc<Mutex<StateQuery<T>>> {
        Arc::new(Mutex::new(StateQuery {
            //list : Vec::new<Box<TransportIn<T>>>(),
            list : Vec::new(),
        }))
    }
}

fn createIdentity<T>() -> Box<Fn(T) -> T> {
    Box::new(|argin: T| -> T {
        argin
    })
}

struct Transport<T, R> {
    query     : Arc<Mutex<StateQuery<T>>>,
    receiver  : Arc<Receiver<R>>,
    transform : Box<Fn(T) -> R>,
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

//TODO - dodać implementacja TransportOut dla Receiver

impl<R, T> TransportOut<R> for Transport<T, R> {
    fn ready(self) {
    }
}

impl<R> Receiver<R> {
    
    fn new() -> Arc<Receiver<R>> {
        Arc::new(Receiver{
            mutex : Mutex::new(ReceiverInner::new()),
            cond  : Condvar::new(),
        })
    }
}

struct ReceiverInner<R> {
    list  : Vec<Box<TransportOut<R>>>,
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
    
    let ch = chan::<String>();
    
    println!("test ... zx");
}
