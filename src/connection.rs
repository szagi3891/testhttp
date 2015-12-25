use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
use std::str;
use std::collections::HashMap;
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
	
    //WaitingForDataUser([u8; 2048], usize),
	WaitingForDataUser([u8; 2048], usize),
    
													//oczekiwanie na wygenerowanie danych z odpowiedzią od serwera
    WaitingForDataServer,

													//siedzą dane gotowe do wysłania dla użytkownika
    DataToSendUser(Vec<u8>, usize),
	
	Close,
}

pub enum ConnectionTransform {
    None,
	Continue,
    Close,
    Write,
    Read,
}

pub struct Connection (TcpStream, bool, ConnectionMode);

								//0 - socket
								//1 - keep alive
								//2 - current mode

impl Connection {

    pub fn new(stream: TcpStream) -> Connection {
	
        Connection(stream, false, ConnectionMode::WaitingForDataUser([0u8; 2048], 0))
	
		//Connection(stream, ConnectionMode::WaitingForDataUser(Vec::with_capacity(2048), 0))
    }
    
    pub fn ready(mut self, events: EventSet) -> (Connection, ConnectionTransform) {
		
        match self {
			
            Connection(mut stream, mut kepp_alive, ConnectionMode::WaitingForDataUser(mut buf, mut done)) => {
				
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
										return (Connection(stream, kepp_alive, ConnectionMode::DataToSendUser(resp_vec, 0)), ConnectionTransform::Continue);
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
					
				}
				
				//trzeba też ustawić jakiś timeout czekania na dane od użytkownika
				
                (Connection(stream, kepp_alive, ConnectionMode::WaitingForDataUser(buf, done)), ConnectionTransform::None)
                            
            }
			
            Connection(stream, mut kepp_alive, ConnectionMode::WaitingForDataServer) => {
                
                (Connection(stream, kepp_alive, ConnectionMode::WaitingForDataServer), ConnectionTransform::None)
            }
			
            Connection(mut stream, mut kepp_alive, ConnectionMode::DataToSendUser(mut str, mut done))  => {
				
				if events.is_writable() {
					
					match stream.try_write(&str[done..str.len()]) {
						
						Ok(Some(size)) => {
							
							if size > 0 {
								
								done = done + size;
								
																//send all data to browser
								if done == str.len() {
									
									if kepp_alive == true {
										
										//keep connection
										//TODO
										
									} else {
											//close connection
										
										return (Connection(stream, kepp_alive, ConnectionMode::Close), ConnectionTransform::Close)
									}
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
					
					//jeśli udany zapis, to zmień stan na oczekiwanie danych od użytkownika lub zamknij to połączenie
					
					//TODO - jeśli już wysłano wszystkie dane z odpowiedzą
						//jeśli jest keep alive nie zachowaj połaczenie
						//jeśli keep alive false, zamknij połączenie
				}
				
				(Connection(stream, kepp_alive, ConnectionMode::DataToSendUser(str, done)), ConnectionTransform::None)
            },
			
			
			Connection(mut stream, mut kepp_alive, ConnectionMode::Close)  => {
				
				(Connection(stream, kepp_alive, ConnectionMode::Close), ConnectionTransform::None)
			}
        }
    }
}

