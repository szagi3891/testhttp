#![feature(mpsc_select)]


extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;

use std::sync::mpsc::{channel};
use simple_signal::{Signals, Signal};

mod miohttp;


fn main() {
    
	
	println!("Hello, world! - 127.0.0.1:13265");
	
	
	println!("TODO - zrobić pętlę na czytaniu danych ?");
    println!("TODO - zrobić pętlę na pisaniu danych ?");
    
    
    let (tx_request, rx_request) = channel::<String>();
		
    
    miohttp::server::MyHandler::new(&"127.0.0.1:13265".to_string(), tx_request);
    
	
	let (ctrl_c_tx, ctrl_c_rx) = channel();
	
	Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
    
        println!("catch ctrl+c");
        
        ctrl_c_tx.send(()).unwrap(); 
    });
	
    
	let mut count = 0;
    
	
	loop {
        
		select! {
			
			_ = ctrl_c_rx.recv() => {
				
				println!("shoutdown");
				return;
			},
			
			//(request, chan_response) = rx_request.recv() => {
            
            conn = rx_request.recv() => {
				
                if count > 20 {
                    return
                }
                
                count = count + 1;
                
				println!("new connection to handle : {:?}", conn);
                
                //formuj odpowiedź
                
                //wyślij odpowiedź na kanał zwrotny
                

                //formatuj obiekt Response            
                /*

                let time_current = time::get_time();

                //TODO - test response
                let response = format!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\n\r\nHello user: {} - {}", time_current.sec, time_current.nsec);

                let mut resp_vec: Vec<u8> = Vec::new();

                for byte in response.as_bytes() {
                    resp_vec.push(byte.clone());
                }

                //TODO - testowa odpowiedź
                Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(resp_vec, 0))
                */
                
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
