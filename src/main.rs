#![feature(mpsc_select)]


extern crate mio;
extern crate http_muncher;
extern crate simple_signal;


use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use simple_signal::{Signals, Signal};

mod token_gen;
mod connection;
mod server;


fn main() {
    	
    println!("Hello, world! - 127.0.0.1:13265");
	
	
    let new_conn_chan = server::MyHandler::new(&"127.0.0.1:13265".to_string());
    
	
	let (ctrl_c_tx, ctrl_c_rx) = channel();
	
	Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
    
        println!("złapałem ctrl+c");
        
        ctrl_c_tx.send(()); 
    });
	
	
	
	loop {
        
		select! {
			
			_ = ctrl_c_rx.recv() => {
				
				println!("rozpoczynam procedurę wyłączania komponentów serwerowych");
				return;
			},
			
			conn = new_conn_chan.recv() => {
				
				println!("odebrano nowe połączenie do obsłużenia");
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

	
	

//use deeply::nested::function as other_function;
// This is equivalent to `use deeply::nested::function as function

//use super::function as root_function;

//use self::cool::function as my_cool_function;
// ===
//use cool::function as root_cool_function;




/*                        thread::spawn(move || {
                            // some work here

                                                            //5 sekund
                            thread::sleep(Duration::new(5, 0));
*/
//                        });

            /*
            match self.hash.get_mut(&token) {

                Some(stream) => {

                    if events.is_readable() {

                        println!("czytam");


                        let mut buf = [0u8; 2048];
						
                        match stream.try_read(&mut buf) {
                        //match Strem.read(&mut buf) {

                            Ok(Some(size)) => {

                                println!("odczytano : {}", size);

                            }
                            Ok(None) => {
                                println!("brak danych");
                            }

                            Err(err) => {
                                println!("błąd czytania ze strumienia {:?}", err);
                            }
                        }
                    }

                    if events.is_writable() {

                        println!("piszę");

                        //fn write(&mut self, buf: &[u8]) -> Result<usize>

                        let response = std::fmt::format(format_args!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: application/xhtml+xml; charset=utf-8\r\n\r\nCześć czołem"));

                        stream.try_write(response.as_bytes()).unwrap();	
                    }
                }
                None => {
                    println!("brak strumienia");
                }
            }
            */
