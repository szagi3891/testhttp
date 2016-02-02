use std::sync::{Arc, Mutex};
use std::thread;
use std::io;
use std::io::prelude::*;


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
//Valueout
//Receiver


//TODO - vec zamienić na kolejkę (potrzebne metody shift i pop)
//TODO - przy tworzeniu pierwszego transportu, trzeba obsłużyć klonowanie receiver-a


fn chan<T: 'static + Send>() -> (Sender<T>, Receiver<T>) {
    
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



fn createIdentity<T>() -> Box<Fn(T) -> T + Send> {
    Box::new(|argin: T| -> T {
        argin
    })
}




fn main() {
    
    let (sender, mut recivier) = chan::<u32>();
    
    
    println!("wysyłam");
    sender.send(32);
    println!("wysłałem");
    
    thread::spawn(move||{
        
        println!("odbieram");
        let from_channel = recivier.get();
        println!("wartość z kanału: {}", from_channel);
    });
    
        
                                //czekaj na ctrl+C
    let stdin = io::stdin();
    for _ in stdin.lock().lines() {     //line
        //println!("{}", line.unwrap());
    }
    
}


        
/*
use std::thread;

trait Foo {
    fn foo(&self);
}

struct Baz {
    pub data : Box<Foo + Send>
}

fn Bar(baz : Baz) {
    thread::spawn(move || {baz.data.foo()});
}

fn main() {}
*/