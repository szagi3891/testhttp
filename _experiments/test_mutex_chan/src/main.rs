use std::sync::{Arc, Mutex};

mod sender;
mod query;
mod transport;
mod receiver;
mod outvalue;

use sender::Sender;
use receiver::Receiver;
use query::Query;
use transport::Transport;
use outvalue::Outvalue;
//Sender
//Query
//Transport
//Receiver


//TODO - vec zamienić na kolejkę (potrzebne metody shift i pop)
//TODO - przy tworzeniu pierwszego transportu, trzeba obsłużyć klonowanie receiver-a


fn chan<T: 'static>() -> (Sender<T>, Receiver<T>) {
    
    let query : Arc<Mutex<Query<T>>> = Query::new();
    let outvalue                     = Outvalue::new();
    let receiver : Receiver<T>       = Receiver::new(outvalue.clone());
    let sender                       = Sender::new(query.clone());
    
    let transport = Transport {
        query    : query,
        outvalue : outvalue.clone(),
        transform : createIdentity::<T>(),
    };
        
    {
        let mut inner = outvalue.lock().unwrap();
        inner.list.push(Box::new(transport));
    }
    
    (sender, receiver)
}



fn createIdentity<T>() -> Box<Fn(T) -> T> {
    Box::new(|argin: T| -> T {
        argin
    })
}




fn main() {
    
    let (sender, recivier) = chan::<u32>();
    
    sender.send(32);
    
    println!("test ... zx");
}
