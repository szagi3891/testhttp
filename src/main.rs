#![feature(mpsc_select)]


extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;

use std::sync::mpsc::{channel};
use simple_signal::{Signals, Signal};

mod miohttp;
use miohttp::request;
use miohttp::response;
//miohttp::request

use std::thread;
//use std::time::Duration;

use std::io::prelude::Read;
use std::fs::File;
use std::path::Path;

fn main() {
    
	
	//println!("Hello, world! - 127.0.0.1:2222");
	
	
	println!("TODO - zrobić pętlę na czytaniu danych ?");
    println!("TODO - zrobić pętlę na pisaniu danych ?");
    
    //mpsc::Sender<(request::Request, mio::Sender<response::Response>)>
    let (tx_request, rx_request) = channel::<(request::Request, mio::Token, mio::Sender<(mio::Token, response::Response)>)>();
		
    
    miohttp::server::MyHandler::new(&"127.0.0.1:2222".to_string(), 4000, 4000, tx_request);
    
	
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
				
				match to_handle {
					
					Ok((req, token, resp_chanel)) => {
						
                        process_request(req, token, resp_chanel)
					}
					
					Err(err) => {
						
						println!("error get from channel {:?}", err);
					}
				}
			}
		}
	}	
}

fn process_request(req: request::Request, token: mio::Token, resp_chanel: mio::Sender<(mio::Token, response::Response)>) {
    
    //	thread::sleep(Duration::new(3, 0));
    
    thread::spawn(move || {
        
        let path_str = "../../static".to_string() + req.path.trim();
        let path     = Path::new(&path_str);
        
        match File::create(path) {
            Ok(mut file) => {
                
                let mut content = String::new();
                file.read_to_string(&mut content);
                
                println!("{:?} {:?}", file, content);
            }
            Err(err) => {
            }
        }
        
        println!("przetwarzam {:?} {:?}", req, path);
        
        let time_current = time::get_time();
        let response_body = format!("Hello user : {} - {}", time_current.sec, time_current.nsec);
        
        let resp = response::Response::create(response::Code::Code200, response::Type::Html, response_body);
        let _    = resp_chanel.send((token, resp));
        
    });
    // -> ../../static
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
