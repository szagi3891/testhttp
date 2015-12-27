#![feature(mpsc_select)]


extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;

use std::sync::mpsc::{channel};
use simple_signal::{Signals, Signal};

mod token_gen;
mod connection;
mod server;


fn main() {
    
	
	println!("Hello, world! - 127.0.0.1:13265");
	
	
	println!("TODO - zrobić pętlę na czytaniu danych ?");
    println!("TODO - zrobić pętlę na pisaniu danych ?");
    
    
    let (tx_request, rx_request) = channel::<String>();
		
    
    server::MyHandler::new(&"127.0.0.1:13265".to_string(), tx_request);
    
	
	let (ctrl_c_tx, ctrl_c_rx) = channel();
	
	Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
    
        println!("catch ctrl+c");
        
        ctrl_c_tx.send(()).unwrap(); 
    });
	
	
	
	loop {
        
		select! {
			
			_ = ctrl_c_rx.recv() => {
				
				println!("shoutdown");
				return;
			},
			
			conn = rx_request.recv() => {
				
				println!("new connection to handle : {:?}", conn);
			}
		}
	}
	
}



/*
mod api1;
mod api2;
mod api3;

mod async;

fn main() {
    
    println!("test asyunchroniczności");
    
    async::test();

}
*/


/*
extern crate simple_signal;

mod thread;

fn main() {
    
    println!("test panic-a w wątkach");
    
    thread::test();

}
*/
