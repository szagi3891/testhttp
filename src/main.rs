/*
extern crate mio;

use mio::{EventLoop, io, buf};

fn main() {
    start().assert("The event loop could not be started");
}

fn start() -> MioResult<EventLoop> {
    // Create a new event loop. This can fail if the underlying OS cannot
    // provide a selector.
    let event_loop = try!(EventLoop::new());

    // Create a two-way pipe.
    let (mut reader, mut writer) = try!(io::pipe());

    // the second parameter here is a 64-bit, copyable value that will be sent
    // to the Handler when there is activity on `reader`
    try!(event_loop.register(&reader, 1u64));

    // kick things off by writing to the writer side of the pipe
    try!(writer.write(buf::wrap("hello".as_bytes())));

    event_loop.run(MyHandler)
}

struct MyHandler;

impl Handler for MyHandler {
    fn readable(&mut self, _loop: &mut EventLoop, token: u64) {
        println!("The pipe is readable: {}", token);
    }
}
*/



//use mio::*;
use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};

use mio::tcp::{TcpListener, TcpStream};
use std::collections::HashMap;

use std::thread;
use std::time::Duration;

extern crate mio;

/*
	keep alive
	kompresja
	
	utworzenie soketu nasłuchującego

	4 nowe event loopy
		nowy event lopp z pożyczeniem tego soketu

	wysyłają kanałem informację o requestach do przetworzenia
		request :
			request		- request - do obsłużenia
			time		- czas zapytania
			kanał zwrotny - na który zostanie przesłana odpowiedź do przesłania
*/

fn main() {

	//token generator
	//kanałem można pobierać nowe tokeny
	
	let mut event_loop = EventLoop::new().unwrap();
	
	
	const SERVER: Token = Token(0);
	
	let addr = "127.0.0.1:13265".parse().unwrap();
	
	let server = TcpListener::bind(&addr).unwrap();
	
	
	event_loop.register(&server, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();
	
	
	
	// Define a handler to process the events
	struct MyHandler {
		server   : TcpListener,
		hash     : HashMap<Token, TcpStream>,
		count    : usize,
	}
	
	impl Handler for MyHandler {
		
		type Timeout = ();
		type Message = ();
		
		fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
			
			match token {
				
				SERVER => {
					
					println!("serwer się zgłosił");
					
					// Accept and drop the socket immediately, this will close
					// the socket and notify the client of the EOF.
					//let _ = server.accept();
					
					match self.server.accept() {
						
						Ok(Some((stream, addr))) => {
							
							let tok = Token(self.count);
							
							self.count = self.count + 1;
							
							event_loop.register(&stream, tok, EventSet::all(), PollOpt::edge());
							//EventSet::readable(), EventSet::writable()
							
							self.hash.insert(tok, stream);
							
							println!("nowe połączenie : {}", addr);
						}
						
						Ok(None) => {
							
							println!("brak nowego połączenia");
						}
						
						Err(e) => {
							
							println!("coś poszło nie tak jak trzeba {}", e);
						}
					}
				}
				_ => {
					
					if events.is_writable() {
						
						match self.hash.remove(&token) {
							
							Some(mut stream) => {
								
								println!("strumień : {:?}", &token);
								
								thread::spawn(move || {
									// some work here
									
																	//5 sekund
									thread::sleep(Duration::new(5, 0));
									
									println!("strumień zapisuję : {:?}", &token);
									
									let response = std::fmt::format(format_args!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\n\r\nCześć czołem"));
								
        							stream.try_write(response.as_bytes()).unwrap();	
								});
								
								
							}
							None => {
								println!("Brak strumienia pod tym hashem: {:?}", &token);
							}
						}
						return;
					}
					
					/*
					match self.hash.get_mut(&token) {
						
						Some(stream) => {
							
							if events.is_readable() {
								
								println!("czytam");
								
								
								let mut buf = [0u8; 2048];
								
								//let mut buf = ByteBuf::mut_with_capacity(2048);
								//let mut buf: String = String::new();
								
								//match Strem.recv_from(buf) {
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
				}
			}
			
			//println!("w ready");
		}
	}
	
	
    println!("Hello, world! - 3");
	
	
	// Start handling events
	event_loop.run(&mut MyHandler{server: server, hash: HashMap::new(), count:1}).unwrap();

	println!("po starcie");
}
