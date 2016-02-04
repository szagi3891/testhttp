use std::sync::{Arc, Mutex};
use std::thread;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

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


fn chan<T: 'static + Clone + Send>() -> (Sender<T>, Receiver<T>) {
    
    let query : Arc<Mutex<Query<T>>> = Query::new();
    let outvalue                     = Outvalue::new();
    let receiver : Receiver<T>       = Receiver::new(outvalue.clone());
    let sender                       = Sender::new(query.clone());
    
    let transport = Transport {
        query    : query,
        outvalue : outvalue.clone(),
        transform : create_identity::<T>(),
    };
        
    {
        let mut inner = outvalue.mutex.lock().unwrap();
        inner.list.push_back(Box::new(transport));
    }
    
    (sender, receiver)
}



fn create_identity<T>() -> Box<Fn(T) -> T + Send> {
    Box::new(|argin: T| -> T {
        argin
    })
}


//TODO - niepotrzebnie jest teraz klonowany arc po to żeby zmieniać zawartość którą posiada mutex
//TODO - zrobić tak, żeby nie trzeba było definiować recivier-a jako mutowalnego



fn main() {
    
    let (sender, recivier) = chan::<u32>();
    
    
    println!("wysyłam");
    sender.send(32);
    sender.send(33);
    println!("wysłałem");
    
    thread::spawn(move||{
        
        println!("odbieram");
        
        loop {
            let from_channel = recivier.get();
            println!("wartość z kanału: {}", from_channel);
        }
    });
    
    sleep(Duration::new(1, 0));    
    sender.send(34);
    sleep(Duration::new(1, 0));
    sender.send(35);
    sleep(Duration::new(1, 0));
    sender.send(36);
    sender.send(37);
    sender.send(38);
        
    
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