#![feature(mpsc_select)]
#![feature(box_syntax, box_patterns)]

extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;

mod async;
mod miohttp;
mod statichttp;

use std::thread;
use std::sync::mpsc::{channel};
use simple_signal::{Signals, Signal};
use miohttp::request;
use miohttp::response;
use std::io;
use std::boxed::FnBox;

fn main() {
    
	let addres = "127.0.0.1:2222";
    
	println!("server running - {}", &addres);
	
    
    let (tx_request, rx_request) = channel::<request::Request>();
    
    miohttp::server::MyHandler::new(&addres.to_string(), 4000, 4000, tx_request);
    
	
	let (ctrl_c_tx, ctrl_c_rx) = channel::<()>();
	
	Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
    
        println!("catch ctrl+c");
        
        ctrl_c_tx.send(()).unwrap(); 
    });
	
    /*
        workery
            api
            
        worker od plików
        
        worker od bazy danych
    */
    
    /*
    
        model uproszczony
        
        worker plików
            pytanie o plik - kanałem
            odpowiedź o plik - kanałem, odpowiada clouserem do uruchomienia oraz danymi tego pliku
            dane pliku współdzielone za pomocą ARC (tylko do odczytu)
            
        proces workera ogólnego (w pełni asynchronicznego)
            tworzy nowy obiekt api (z namiarami na kanały workera plików)
            odbiera request - uruchamia główną metodę api
            odbiera clousera - uruchamia go
    */
    
    let (tx_files_path, rx_files_path) = channel::<(String, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>();
    let (tx_files_data, rx_files_data) = channel::<(Result<Vec<u8>, io::Error>, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>();
        
    thread::spawn(move || {
        
        statichttp::run(rx_files_path, tx_files_data);
    });
    
    
	loop {
        
		select! {
			
			_ = ctrl_c_rx.recv() => {
				
				println!("shoutdown");
				return;
			},
			
            to_handle = rx_request.recv() => {
				
				match to_handle {
					
					Ok(request) => {
						
                        
                        let path_str = "./static".to_owned() + request.path.trim();
                        
                        //versia 1
                        
                        tx_files_path.send((path_str, Box::new(move|data: Result<Vec<u8>, io::Error>|{
                            
                            match data {
                                
                                Ok(buffer) => {
                                    
                                    let buffer = buffer.to_owned();
                            
                                    let response = response::Response::create_from_buf(response::Code::Code200, response::Type::Html, buffer);
                                    request.send(response);
                                }
                                
                                Err(err) => {
                                    println!("err: {}", err);
                                }
                            }
                        })));
					}
					
					Err(err) => {
						
						println!("error get from channel {:?}", err);
					}
				}
			},
        
            data = rx_files_data.recv() => {
                
                match data {
                    Ok((result, callback)) => {
                        
                        callback(result);
                        
                    }
                    Err(err) => {
                        println!("error ...");
                    }
                }
                
                //println!("odebrano dane pliku {:?}", files_data);
            }
		}
	}	
}

