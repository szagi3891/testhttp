use mio::{Token, EventLoop, EventSet, PollOpt, TryRead, TryWrite};
use mio::tcp::{TcpStream};
use server::MyHandler;
use httparse;
use time;


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
    
	
	
    pub fn ready(self, events: EventSet, tok: Token, event_loop: &mut EventLoop<MyHandler>) -> (Connection, bool) {
		
		if events.is_error() {
			println!("EVENT ERROR {}", tok.as_usize());
		}
		
		let (new_connection, is_close) = self.transform(events);
		
		
		let new_connection = new_connection.set_events(event_loop, tok);
		
		
		if events.is_hup() {
			
			println!("EVENT HUP - prepending - {} {:?}", tok.as_usize(), events);
			
			
			match new_connection {
				
				Connection(stream, keep_alive, _) => {
					
					println!("EVENT HUP - close - {} {:?}", tok.as_usize(), events);
					
					return (Connection(stream, keep_alive, ConnectionMode::Close), true);
				}
			}
		}
		
		
		
		(new_connection, is_close)
    }


	fn set_events(self, event_loop: &mut EventLoop<MyHandler>, token: Token) -> Connection {
		
		let base_event = EventSet::error() | EventSet::hup();
		let pool_opt   = PollOpt::edge();	// | PollOpt::oneshot();
		//let pool_opt   = PollOpt::level();
			
		match self {
			
			Connection(stream, keep_alive, ConnectionMode::WaitingForDataUser(buf, done)) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! set mode event : 1");
				
				event_loop.reregister(&stream, token, base_event | EventSet::readable(), pool_opt).unwrap();
				
				Connection(stream, keep_alive, ConnectionMode::WaitingForDataUser(buf, done))
			}
			
			Connection(stream, keep_alive, ConnectionMode::WaitingForDataServer) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! set mode event : 2");
				
				event_loop.reregister(&stream, token, base_event, pool_opt).unwrap();
				
				Connection(stream, keep_alive, ConnectionMode::WaitingForDataServer)
			}
			
			Connection(stream, keep_alive, ConnectionMode::DataToSendUser(str, done)) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! set mode event : 3");
				
				event_loop.reregister(&stream, token, base_event | EventSet::writable(), pool_opt).unwrap();
				
				Connection(stream, keep_alive, ConnectionMode::DataToSendUser(str, done))
		   	}
			
		    Connection(stream, keep_alive, ConnectionMode::Close) => {
				
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! set mode event : 4");
				
				event_loop.deregister(&stream).unwrap();
					
				Connection(stream, keep_alive, ConnectionMode::Close)
			}
		}
	}
	
	fn transform(self, events: EventSet) -> (Connection, bool) {
		
        match self {
			
            Connection(mut stream, keep_alive, ConnectionMode::WaitingForDataUser(mut buf, mut done)) => {
				
				if events.is_readable() {
					
                    
					let total = buf.len();
                    
                    println!("total count {}", &done);
                    
					match stream.try_read(&mut buf[done..total]) {
						
						Ok(Some(size)) => {
                            
							if size > 0 {
								
								done = done + size;

								println!("read : {}", size);


								let mut headers = [httparse::EMPTY_HEADER; 100];
								let mut req     = httparse::Request::new(&mut headers);

								match req.parse(&buf) {

									Ok(httparse::Status::Complete(size_parse)) => {
										
										println!("parse ok, get count {}, parse count {}", done, size_parse);
										
										println!("method : {:?}", req.method);
										println!("path : {:?}", req.path);
										println!("version : {:?}", req.version);
										//println!("headers : {:?}", req.headers);
										
										for header in req.headers {
											let str_header = String::from_utf8_lossy(header.value);
											println!("  {} : {}", header.name, str_header);
										}
										
										//TODO - get info about keep alive
										
										
										let time_current = time::get_time();
											
										//TODO - test response
										let response = format!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\n\r\nHello user: {} - {}", time_current.sec, time_current.nsec);
										
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
												println!("error parse {:?}", err);
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
							println!("no data");
						}
                        
						Err(err) => {
							println!("error read from socket {:?}", err);
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
				
                (Connection(stream, keep_alive, ConnectionMode::WaitingForDataUser(buf, done)), false)
            }
			
            Connection(stream, keep_alive, ConnectionMode::WaitingForDataServer) => {
                
                (Connection(stream, keep_alive, ConnectionMode::WaitingForDataServer), false)
            }
			
            Connection(mut stream, keep_alive, ConnectionMode::DataToSendUser(str, mut done))  => {
				
				if events.is_writable() {
					
					match stream.try_write(&str[done..str.len()]) {
						
						Ok(Some(size)) => {
							
							println!("write data count='{}'", size);
							
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
									//println!("XXXXXXXXX");
									panic!("something went wrong");
								}
							}
						}
						
						Ok(None) => {
							
							println!("empty write");
						}
						
						Err(err) => {
							
							println!("error write to socket {:?}", err);
						}
					}
				}
				
				(Connection(stream, keep_alive, ConnectionMode::DataToSendUser(str, done)), false)
            },
			
			
			Connection(stream, keep_alive, ConnectionMode::Close)  => {
				
				(Connection(stream, keep_alive, ConnectionMode::Close), false)
			}
        }
	}
}

