use std::thread;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

mod chan;
mod sender;
mod query;
mod transport;
mod receiver;
mod outvalue;

use chan::Chan;

//TODO - vec zamienić na kolejkę (potrzebne metody shift i pop)
//TODO - przy tworzeniu pierwszego transportu, trzeba obsłużyć klonowanie receiver-a

//TODO - niepotrzebnie jest teraz klonowany arc po to żeby zmieniać zawartość którą posiada mutex

//TODO - wreszcie, zrobić selecta
//na zasadzie, new::reciver<RR>, Fn(T) -> RR, recivier<T> zjadany



fn main() {
    
    let chan = Chan::new();
    
    let receiver1 = chan.receiver();
    let receiver2 = chan.receiver();
    let receiver3 = chan.receiver();
    
    let sender1   = chan.sender();
    let sender2   = chan.sender();
    let sender3   = chan.sender();
    
    
    thread::spawn(move||{
        
        let mut count = 1000;
        
        loop {
            sender1.send(count.clone());
            sleep(Duration::from_millis(300));
            count = count + 1;
        }
    });
    
    thread::spawn(move||{
        
        let mut count = 1;
        
        loop {
            sender2.send(count.clone());
            sleep(Duration::from_millis(1000));
            count = count + 1;
        }
    });
    
    thread::spawn(move||{
        
        let mut count = 1000000;
        
        loop {
            sender3.send(count.clone());
            sleep(Duration::from_millis(3000));
            count = count + 1;
        }
    });
    
    thread::spawn(move||{
        
        loop {
            let from_channel = receiver1.get();
            println!("wątek1: wartość z kanału: {}", from_channel);
        }
    });

    thread::spawn(move||{
        
        loop {
            let from_channel = receiver2.get();
            println!("wątek2: wartość z kanału: {}", from_channel);
        }
    });
    
    thread::spawn(move||{
        
        loop {
            let from_channel = receiver3.get();
            println!("wątek3: wartość z kanału: {}", from_channel);
        }
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