use std::sync::{Arc, Mutex, Condvar};
use std::collections::LinkedList;

fn chan<'a, T: 'a>() -> (Sender<T>, Arc<Receiver<'a, T>>) {
    
    let query : Arc<Mutex<StateQuery<T>>> = StateQuery::new();
    let receiver : Arc<Receiver<T>>       = Receiver::new();
    let sender                            = Sender::new(query.clone());
    
    let transport : Transport<'a, T, T> = Transport {
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

struct Transport<'a, T: 'a, R: 'a> {
    query     : Arc<Mutex<StateQuery<T>>>,
    receiver  : Arc<Receiver<'a, R>>,
    transform : Box<Fn(T) -> R>,
}
    
trait TransportIn<T> {      //T:Sized
    fn send(self, T);       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

trait TransportOut<R> {
    fn ready(self);
}

struct Receiver<'a, R> {
    mutex : Mutex<ReceiverInner<'a, R>>,
    cond  : Condvar,
}

//TODO - dodać implementacja TransportOut dla Receiver

impl<'a, R, T> TransportOut<R> for Transport<'a, T, R> {
    fn ready(self) {
    }
}

impl<'a, R> Receiver<'a, R> {
    
    fn new() -> Arc<Receiver<'a, R>> {
        Arc::new(Receiver{
            mutex : Mutex::new(ReceiverInner::new()),
            cond  : Condvar::new(),
        })
    }
}
   
struct ReceiverInner<'a, R> {
    list  : Vec<Box<TransportOut<R> + 'a>>,
}

impl<'a, R> ReceiverInner<'a, R> {
    fn new() -> ReceiverInner<'a, R> {
        
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
