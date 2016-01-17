extern crate comm;

use std::thread;
use std::time::Duration;
use comm::mpmc::bounded::Channel;
use comm::select::Select;
use comm::select::Selectable;

//type Message = i32;


fn main() {
    println!("Creating channels...");

    // Create a two bounded MPMC channels.
    let chan_int = Channel::new(100);
    let chan_float = Channel::new(100);
    
    // Thread sending integers to chan_int
    let send_int = chan_int.clone();
    let t1 = thread::spawn(move || {
        for i in 0i32..100 {
            thread::sleep(Duration::from_millis(14));
            println!("--> INT {}", i);
            send_int.send_sync(i).unwrap();
        }
    });

    // Thread sending floats to chan_float
    let send_float = chan_float.clone();
    let t2 = thread::spawn(move || {
        for i in 0i32..100 {
            let f = i as f64;
            thread::sleep(Duration::from_millis(22));
            println!("--> FLOAT {}", f/100.0);
            send_float.send_sync(f/100.0).unwrap();
        }
    });

    for i in 0i32..30 {
        let recv_int = chan_int.clone();
        let recv_float = chan_float.clone();
        thread::spawn(move || {
            let select = Select::new();
            select.add(&recv_int);
            select.add(&recv_float);
            loop {
                for &mut id in select.wait(&mut [0, 0]) {
                    if id == recv_int.id() { println!("<-- Thread {:2} received int  : {:4}", i, recv_int.recv_sync().unwrap()); }
                    else if id == recv_float.id() { println!("<-- Thread {:2} received float: {:.2}", i, recv_float.recv_sync().unwrap()); }
                }
                // Worker is busy for 500 ms computing received data.
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

    let _ = t1.join();
    println!("Sending integers ended...");
    let _ = t2.join();
    println!("Sending floats ended, waiting 3 seconds...");
    thread::sleep(Duration::from_millis(3000));
}
