use std::thread;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

mod types;
mod fnconvert;
mod chan;
mod sender;
mod query;
mod transport;
mod transformer;
mod receiver;
mod outvalue;

use chan::Chan;

//TODO - niepotrzebnie jest teraz klonowany arc po to żeby zmieniać zawartość którą posiada mutex

//TODO - wreszcie, zrobić selecta
//na zasadzie, new::reciver<RR>, Fn(T) -> RR, recivier<T> zjadany

//TODO
//LinkedList<Box<TransportOut<R> + Send>>, - opakować tą listę typem zewnętrznym którego używać we wszystkich miejscach
//udostniępniać tylko metodę push oraz pop (będą one dbały o właściwy kierunek)

enum Out {
    Result1(u64),
    Result2(u64),
}

fn main() {
    
    let (sender1, receiver1) = Chan::new().couple();
    let (sender2, receiver2) = Chan::new().couple();
    
    
    
    thread::spawn(move||{
        
        let mut count = 1;
        
        loop {
            sender1.send(count.clone());
            sleep(Duration::from_millis(150));
            count = count + 1;
        }
    });
    
    
    thread::spawn(move||{
        
        let mut count = 1000;
        
        loop {
            sender2.send(format!("wartość druga {}", count.clone()));
            sleep(Duration::from_millis(300));
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
    
    
    /*
    let select = Chan::Select::<Out>();
    
    select.add(receiver1, Box::new(|value: u64| -> Out {
        Out::Result1(value)
    }));
    
    select.add(receiver2, Box::new(|value: u64| -> Out {
        Out::Result2(value)
    }));
    
    thread::spawn(move||{
        
        loop {
            let from_channel = select.get();
            println!("wątek select: wartość z kanału: {}", from_channel);
        }
    });
    */
    
    
                                //czekaj na ctrl+C
    let stdin = io::stdin();
    for _ in stdin.lock().lines() {     //line
        //println!("{}", line.unwrap());
    }
    
}
