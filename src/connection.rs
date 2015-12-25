use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
use std::str;
use std::collections::HashMap;
use server::MyHandler;
use httparse;


/*
struct request {
    //parser
    //metody dostępowe
}

http://seanmonstar.com/
	info o bezstanowości httparse

https://github.com/hyperium/hyper/blob/master/src/buffer.rs
	sprawdzić jak hyper sobie radzi z parsowaniem danych ...

https://github.com/nbaksalyar/rust-chat/blob/part-1/src/main.rs#L2
	dobrze zaimplementowane mio
*/

/*
So really, 'allocation-free' means, make any allocations you want beforehand, and then give me a slice. (Hyper creates a stack array of [Header; 100], for instance).

https://github.com/seanmonstar/httparse

				httpparse w hyper
https://github.com/hyperium/hyper/blob/master/src/http/h1.rs
*/


/*
fn read(stream : &mut TcpStream, total : usize) -> Option<Vec<u8>>
{
	let mut buffer = Vec::with_capacity(total);
	let mut done   = 0;

	unsafe { 
		buffer.set_len(total)
	}

	while done < total {

		if let Ok(count) = stream.read(&mut buffer[done..total]) {
			done += count;    
		} else {
			break;
		}   
	}

	if done == total {
		Some(buffer)
	} else {
		None
	}   
}
*/


enum ConnectionMode {
	
	WaitingForDataUser([u8; 2048], usize),
													//oczekiwanie na wygenerowanie danych z odpowiedzią od serwera
    WaitingForDataServer,
													//siedzą dane gotowe do wysłania dla użytkownika
    DataToSendUser(Vec<u8>, usize),
	
	Close,
}


								//socket, keep alive, current mode
pub struct Connection (TcpStream, bool, ConnectionMode);


impl Connection {

    pub fn new(stream: TcpStream) -> Connection {
	
        Connection(stream, false, ConnectionMode::WaitingForDataUser([0u8; 2048], 0))
    }
    
	
	pub fn set_options(mut self, is_new: bool, event_loop: &mut EventLoop<MyHandler>, token: Token) -> Connection {
		
		//(&stream, EventSet::all(), PollOpt::edge())
		
		let base_event = EventSet::error() | EventSet::hup();
		let pool_opt   = PollOpt::edge() | PollOpt::oneshot();
		//let pool_opt   = PollOpt::level();
		
		/*
		let set_event = |stream, token, event, pool_opt|{
			
			if is_new {
				event_loop.register  (stream, token, event, pool_opt);
			} else {
				event_loop.reregister(stream, token, event, pool_opt);
			}
		};
		*/
			
		match self {
			
			Connection(stream, keep_alive, ConnectionMode::WaitingForDataUser(buf, done)) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! ustawiam tryb : 1");
				
				//set_event(&stream, token, base_event | EventSet::readable(), pool_opt);
				
				
				if is_new {
					event_loop.register  (&stream, token, base_event | EventSet::readable(), pool_opt);
				} else {
					event_loop.reregister(&stream, token, base_event | EventSet::readable(), pool_opt);
				}
				
				
				Connection(stream, keep_alive, ConnectionMode::WaitingForDataUser(buf, done))
			}
			
			Connection(stream, keep_alive, ConnectionMode::WaitingForDataServer) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! ustawiam tryb : 2");
				//set_event(&stream, token, base_event, pool_opt);
				
				
				if is_new {
					event_loop.register  (&stream, token, base_event, pool_opt);
				} else {
					event_loop.reregister(&stream, token, base_event, pool_opt);
				}
				
				Connection(stream, keep_alive, ConnectionMode::WaitingForDataServer)
			}
			
			Connection(stream, keep_alive, ConnectionMode::DataToSendUser(str, done)) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! ustawiam tryb : 3");
				//set_event(&stream, token, base_event | EventSet::writable(), pool_opt);
				
				
				if is_new {
					event_loop.register  (&stream, token, base_event | EventSet::writable(), pool_opt);
				} else {
					event_loop.reregister(&stream, token, base_event | EventSet::writable(), pool_opt);
				}
				
				Connection(stream, keep_alive, ConnectionMode::DataToSendUser(str, done))
		   	}
			
		    Connection(stream, keep_alive, ConnectionMode::Close) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! ustawiam tryb : 4");
				//set_event(&stream, token, base_event, pool_opt);
				
				
				if is_new {
					event_loop.register  (&stream, token, EventSet::none(), pool_opt);
				} else {
					event_loop.reregister(&stream, token, EventSet::none(), pool_opt);
				}
				
				Connection(stream, keep_alive, ConnectionMode::Close)
			}
		}
	}
	/*
	EventSet::readable()
	EventSet::writable()
	EventSet::error()
	EventSet::hup()
	*/
	
	//| PollOpt::oneshot()
	
	
    pub fn ready(mut self, events: EventSet, tok: Token) -> (Connection, bool) {
		
		if events.is_hup() {
			println!("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT {}", tok.as_usize());
			return (Connection(self.0, self.1, ConnectionMode::Close), true);
		}
		
		if events.is_error() {
			println!("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT 2");
		}
		
        match self {
			
            Connection(mut stream, mut keep_alive, ConnectionMode::WaitingForDataUser(mut buf, mut done)) => {
				
				if events.is_readable() {
					
                    
					let total = buf.len();
                    
                    println!("total w pozycji {}", &done);
                    
					match stream.try_read(&mut buf[done..total]) {
						
						Ok(Some(size)) => {
                            
							if size > 0 {
								
								done = done + size;

								println!("odczytano : {}", size);


								let mut headers = [httparse::EMPTY_HEADER; 100];
								let mut req     = httparse::Request::new(&mut headers);

								match req.parse(&buf) {

									Ok(httparse::Status::Complete(size_parse)) => {
										
										println!("ok parsowanie");
										
										println!("method : {:?}", req.method);
										println!("path : {:?}", req.path);
										println!("version : {:?}", req.version);
										println!("headers : {:?}", req.headers);
										
										//TODO - wyciagnij informację na temat keep alive, trzeba uwzględnić tą wartość
										
										
										//TODO - testowa odpowiedź
										let response = "HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\n\r\nCześć czołem";
										
										let mut resp_vec: Vec<u8> = Vec::new();
										
										for byte in response.as_bytes() {
											resp_vec.push(byte.clone());
										}
										
										//TODO - testowa odpowiedź
										return (Connection(stream, keep_alive, ConnectionMode::DataToSendUser(resp_vec, 0)), false);
									}

									Ok(httparse::Status::Partial) => {
										//częściowe parsowanie
									}

									Err(err) => {

										match err {
											httparse::Error::HeaderName => {
												println!("header name");
											}
											_ => {
												println!("błąd parsowania {:?}", err);
											}
										}

										/* HeaderName, HeaderValue, NewLine, Status, Token, TooManyHeaders, Version */
									}
								}

								//uruchom parser
									//jeśli się udało sparsować, to git

								//jeśli osiągneliśmy całkowity rozmiar bufora a mimo to nie udało się sparsować danych
									//to rzuć błędem że nieprawidłowe zapytanie
							}
						}
						
						Ok(None) => {
							println!("brak danych");
						}
                        
						Err(err) => {
							println!("błąd czytania ze strumienia {:?}", err);
						}
					}
					
					
					
					//czytaj, odczytane dane przekaż do parsera
					//jeśli otrzymalismy poprawny obiekt requestu to :
						// przełącz stan tego obiektu połączenia, na oczekiwanie na dane z serwera
						// wyślij kanałem odpowiednią informację o requescie
						// zwróć informację na zewnątrz tej funkcji że nic się nie dzieje z tym połaczeniem
					
					return (Connection(stream, keep_alive, ConnectionMode::WaitingForDataUser(buf, done)), false);
				}
				
				//trzeba też ustawić jakiś timeout czekania na dane od użytkownika
				
                println!("YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY");
				
                (Connection(stream, keep_alive, ConnectionMode::WaitingForDataUser(buf, done)), false)
            }
			
            Connection(stream, mut keep_alive, ConnectionMode::WaitingForDataServer) => {
                
                (Connection(stream, keep_alive, ConnectionMode::WaitingForDataServer), false)
            }
			
            Connection(mut stream, mut keep_alive, ConnectionMode::DataToSendUser(mut str, mut done))  => {
				
				if events.is_writable() {
					
					match stream.try_write(&str[done..str.len()]) {
						
						Ok(Some(size)) => {
							
							if size > 0 {
								
								done = done + size;
								
																//send all data to browser
								if done == str.len() {
									/*
									if keep_alive == true {
										
										//keep connection
										//TODO
										
									} else {
										*/
											//close connection
										
										return (Connection(stream, keep_alive, ConnectionMode::Close), true);
									//}
								} else {
									println!("XXXXXXXXX");
								}
							}
						}
						
						Ok(None) => {
							
							println!("nic się nie zapisało");
						}
						
						Err(err) => {
							
							println!("błąd zapisywania do strumienia {:?}", err);
						}
					}
				}
				
				(Connection(stream, keep_alive, ConnectionMode::DataToSendUser(str, done)), false)
            },
			
			
			Connection(mut stream, mut keep_alive, ConnectionMode::Close)  => {
				
				(Connection(stream, keep_alive, ConnectionMode::Close), false)
			}
        }
    }
}

