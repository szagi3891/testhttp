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

//TODO - niepotrzebnie jest teraz klonowany arc po to żeby zmieniać zawartość którą posiada mutex

//TODO - wreszcie, zrobić selecta
//na zasadzie, new::reciver<RR>, Fn(T) -> RR, recivier<T> zjadany

//TODO
//LinkedList<Box<TransportOut<R> + Send>>, - opakować tą listę typem zewnętrznym którego używać we wszystkich miejscach
//udostniępniać tylko metodę push oraz pop (będą one dbały o właściwy kierunek)


fn main() {
    
    let (sender1, receiver1) = Chan::new().couple();
    let (sender2, receiver2) = Chan::new().couple();
    
    
    //TODO - Trzeba stworzyć wspólny kanał agregujące dane z receiver1 oraz receiver2
    
    /*
    enum Out {
        Result1(T1),
        Result2(T2),
    }
    
    let receiver_out = receiver1.transform(Fn(T1)->Out)
    receiver_out.add(receiver2, Fn(T2)->Out)
    
    receiver_out.get() - zbierać dane będzie od tego momentu z dwóch źródeł
    */
    
    
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
    
    
                                //czekaj na ctrl+C
    let stdin = io::stdin();
    for _ in stdin.lock().lines() {     //line
        //println!("{}", line.unwrap());
    }
    
}
