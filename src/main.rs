#![feature(fnbox)]

extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;
#[macro_use]
extern crate chan;

use std::{io, thread};
use std::boxed::FnBox;
use simple_signal::{Signals, Signal};
use miohttp::{request, response};

mod async;
mod miohttp;
mod statichttp;


fn main() {
    
	let addres = "0.0.0.0:2222";
    
	println!("server running - {}", &addres);
	
    
    let (tx_request, rx_request) = chan::sync(0);       //channel::<request::Request>();
    
    miohttp::server::MyHandler::new(&addres.to_string(), 4000, 4000, tx_request);
    
	
	let (ctrl_c_tx, ctrl_c_rx) = chan::sync(0);        //channel::<()>();
	
	Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
    
        println!("catch ctrl+c");
        
        ctrl_c_tx.send(());
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
    
    let (tx_files_path, rx_files_path) = chan::sync(0);
    let (tx_files_data, rx_files_data) = chan::sync(0);
    
    
    /*
    let (tx_files_path, rx_files_path) = channel::<(String, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>();
    let (tx_files_data, rx_files_data) = channel::<(Result<Vec<u8>, io::Error>, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>();
    */
    
    thread::spawn(move || {
        
        statichttp::run(rx_files_path, tx_files_data);
    });
    
    /*
        TODO - potencjalnie do zastosowania - możliwe wielu konsumerów

        https://github.com/BurntSushi/chan              - github
        http://burntsushi.net/rustdoc/chan/             - dokumentacja
        https://github.com/BurntSushi/chan/issues/2     - przebrnąć
    */
    
	loop {
        
		chan_select! {
			
			ctrl_c_rx.recv() => {
				
				println!("shoutdown");
				return;
			},
			
            rx_request.recv() -> to_handle => {
				
				match to_handle {
					
					Some(request) => {
						
                        
                        let path_str = "./static".to_owned() + request.path.trim();
                        
                        //versia 1
                        
                        println!("ścieżka do zaserwowania {}", path_str);
                        
                        
                        tx_files_path.send((path_str, Box::new(move|data: Result<Vec<u8>, io::Error>|{
                            
                            match data {
                                
                                Ok(buffer) => {
                                    
                                    let buffer = buffer.to_owned();
                            
                                    let response = response::Response::create_from_buf(response::Code::Code200, response::Type::Html, buffer);
                                    request.send(response);
                                }
                                
                                Err(err) => {
                                    
                                    println!("error czytania pliku: {}", err);
                                }
                            }
                        })));
					}
					
					None => {
						
                        //TODO
                        println!("wyparował nadawca requestów");
						//println!("error get from channel {:?}", err);
					}
				}
			},
        
            rx_files_data.recv() -> data => {
                
                println!("odebrało dane z plikiem");
                
                match data {
                    
                    Some((result, callback)) => {    
                        callback.call_box((result,));    
                    }
                    
                    None => {
                        
                        //TODO
                        println!("wyparował nadawca requestów");
                    }
                }
                
                //println!("odebrano dane pliku {:?}", files_data);
            }
		}
	}	
}

