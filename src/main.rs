#![feature(mpsc_select)]


extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;

use std::sync::mpsc::{channel};
use simple_signal::{Signals, Signal};
//use mio::EventLoop;

mod miohttp;
use miohttp::request;
use miohttp::response;
//miohttp::request

//use std::thread;
//use std::time::Duration;


fn main() {
    
	
	//println!("Hello, world! - 127.0.0.1:13265");
	
	
	println!("TODO - zrobić pętlę na czytaniu danych ?");
    println!("TODO - zrobić pętlę na pisaniu danych ?");
    
    //mpsc::Sender<(request::Request, mio::Sender<response::Response>)>
    let (tx_request, rx_request) = channel::<(request::Request, mio::Token, mio::Sender<(mio::Token, response::Response)>)>();
		
    
    miohttp::server::MyHandler::new(&"127.0.0.1:13265".to_string(), 4000, 4000, tx_request);
    
	
	let (ctrl_c_tx, ctrl_c_rx) = channel();
	
	Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
    
        println!("catch ctrl+c");
        
        ctrl_c_tx.send(()).unwrap(); 
    });
	
    
	//let mut count = 0;
    
	
	loop {
        
		select! {
			
			_ = ctrl_c_rx.recv() => {
				
				println!("shoutdown");
				return;
			},
			
            to_handle = rx_request.recv() => {
				
				/*
                if count > 20 {
                    return
                }
                count = count + 1;
                */
				
				match to_handle {
					
					Ok((req, token, resp_chanel)) => {
						
						//thread::spawn(move || {
							
						//	thread::sleep(Duration::new(3, 0));
							
							let time_current = time::get_time();

							//TODO - test response
							let response_body = format!("Hello user: {} - {}", time_current.sec, time_current.nsec);
							let response = format!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\nConnection: keep-alive\r\nContent-length: {}\r\n\r\n{}", response_body.len(), response_body);

							let _ = resp_chanel.send((token, response::Response::from_string(response)));

							println!("przesłano kanał z odpowiedzią : {:?}", req);
						//});
					}
					
					Err(err) => {
						
						println!("error get from channel {:?}", err);
					}
				}
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
