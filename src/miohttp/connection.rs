use mio::{Token, EventLoop, EventSet, PollOpt, TryRead, TryWrite};
use mio::tcp::{TcpStream};
use httparse;
use miohttp::server::MyHandler;
use miohttp::request::Request;



enum ConnectionMode {
	
                                                    //czytanie requestu
	ReadingRequest([u8; 2048], usize),
                                                    //oczekujący sparsowany request
    ParsedRequest(Request),
													//oczekiwanie na wygenerowanie odpowiedzi serwera
    WaitingForServerResponse,
													//wysyłanie odpowiedz
    SendingResponse(Vec<u8>, usize),
                                                    //połączenie do zamknięcia
	Close,
}


                                //typ zdarzeń który jest ustawiony w mio
enum Event {
    Init,
    Write,
    Read,
    None
}

								//socket, keep alive, actuall event set, current mode
pub struct Connection (TcpStream, bool, Event, ConnectionMode);

	//TODO - może warto z połączeniem przechowywać również token ... ?

impl Connection {
    
    
    pub fn new(stream: TcpStream, tok: Token, event_loop: &mut EventLoop<MyHandler>) -> Connection {
	
        let conn = Connection(stream, false, Event::Init, ConnectionMode::ReadingRequest([0u8; 2048], 0));
        
        conn.set_events(event_loop, tok)
    }
    
    
    pub fn in_state_close(&self) -> bool {
        
        match *self {
            
            Connection(_, _, _, ConnectionMode::Close) => {
                true
            }
            _ => {
                false
            }
        }
    }
    
    
    pub fn get_request(self, tok: Token, event_loop: &mut EventLoop<MyHandler>) -> (Connection, Option<Request>) {
        
        let (new_connection, request) = match self {
            Connection(stream, keep_alive, event, ConnectionMode::ParsedRequest(request)) => {
                (Connection(stream, keep_alive, event, ConnectionMode::WaitingForServerResponse), Some(request))
            }
            Connection(stream, keep_alive, event, mode) => {
                (Connection(stream, keep_alive, event, mode), None)
            }
        };
		
		let new_connection = new_connection.set_events(event_loop, tok);
		
		(new_connection, request)
    }
	
    pub fn ready(self, events: EventSet, tok: Token, event_loop: &mut EventLoop<MyHandler>) -> Connection {
		
        
		if events.is_error() {
			println!("EVENT ERROR {}", tok.as_usize());
            panic!("TODO");
		}
		
        
		let new_connection = self.transform(events);
		
        
        let new_connection = match new_connection {

            Connection(stream, keep_alive, event, mode) => {
                
                if events.is_hup() {
			         
                    println!("EVENT HUP - close - {} {:?}", tok.as_usize(), events);
                    
                    Connection(stream, keep_alive, event, ConnectionMode::Close)
                    
                } else {
                    
                    Connection(stream, keep_alive, event, mode)
                }
            }
        };		
		
        
		let new_connection = new_connection.set_events(event_loop, tok);
		
        new_connection
    }
    

	fn set_events(self, event_loop: &mut EventLoop<MyHandler>, token: Token) -> Connection {
		
		let base_event = EventSet::error() | EventSet::hup();
        let pool_opt   = PollOpt::edge() | PollOpt::oneshot();
        
		match self {
			
			Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done)) => {
				
                println!("----------> set mode : WaitingForDataUser");
                
                let event_read = base_event | EventSet::readable();
                
				match event {
                    
                    Event::Init => {
                        println!("----------> register: {:?} {:?}", token, event_read);
                        event_loop.register(&stream, token, event_read, pool_opt).unwrap();
                    }
                    
                    Event::Write => {}
                    
                    _ => {
                        println!("----------> reregister: {:?} {:?}", token, event_read);
                        event_loop.reregister(&stream, token, event_read, pool_opt).unwrap();
                    }
                }
                
                Connection(stream, keep_alive, Event::Write, ConnectionMode::ReadingRequest(buf, done))
			}
			
            Connection(stream, keep_alive, event, ConnectionMode::ParsedRequest(request)) => {
                
                println!("----------> set mode : ParsedRequest");
                
                let event_none = base_event;
                
				match event {
                    
                    Event::Init => {
                        println!("----------> register: {:?} {:?}", token, event_none);
                        event_loop.register(&stream, token, event_none, pool_opt).unwrap();
                    }
                    
                    Event::None => {}
                    
                    _ => {
                        println!("----------> reregister: {:?} {:?}", token, event_none);
                        event_loop.reregister(&stream, token, event_none, pool_opt).unwrap();
                    }
                }
                
                Connection(stream, keep_alive, Event::None, ConnectionMode::ParsedRequest(request))   
            }
            
			Connection(stream, keep_alive, event, ConnectionMode::WaitingForServerResponse) => {
				
				println!("----------> set mode : WaitingForDataServer");
				
                let event_none = base_event;
                
                match event {
                    
                    Event::Init => {
                        println!("----------> register: {:?} {:?}", token, event_none);
                        event_loop.register(&stream, token, event_none, pool_opt).unwrap();
                    }
    
                    Event::None => {}
                    
                    _ => {
                        println!("----------> reregister: {:?} {:?}", token, event_none);
                        event_loop.reregister(&stream, token, event_none, pool_opt).unwrap();
                    }
                }
                
                Connection(stream, keep_alive, Event::None, ConnectionMode::WaitingForServerResponse)
			}
			
			Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(str, done)) => {
				
				println!("----------> set mode : DataToSendUser");
				
                let event_write = base_event | EventSet::writable();
                
				match event {
                    
                    Event::Init => {
                        println!("----------> register: {:?} {:?}", token, event_write);
                        event_loop.register(&stream, token, event_write, pool_opt).unwrap();
                    }
                    
                    Event::Read => {}
                    
                    _ => {
                        println!("----------> reregister: {:?} {:?}", token, event_write);
                        event_loop.reregister(&stream, token, event_write, pool_opt).unwrap();
                    }
                }
                
				Connection(stream, keep_alive, Event::Read, ConnectionMode::SendingResponse(str, done))
		   	}
			
		    Connection(stream, keep_alive, event, ConnectionMode::Close) => {
				
				println!("----------> set mode : Close");
				
                match event {
                    
                    Event::Init => {}
                    
                    _ => {
                        println!("----------> deregister: {:?}", token);
                        event_loop.deregister(&stream).unwrap();
                    }
                }
                
				Connection(stream, keep_alive, Event::None, ConnectionMode::Close)
			}
		}
	}
	
    
	fn transform(self, events: EventSet) -> Connection {
		
        match self {
			
            Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done)) => {
				
                transform_from_waiting_for_user(events, stream, keep_alive, event, buf, done)
            }
			
            Connection(stream, keep_alive, event, ConnectionMode::ParsedRequest(request)) => {
                
                Connection(stream, keep_alive, event, ConnectionMode::ParsedRequest(request))
            }
            
            Connection(stream, keep_alive, event, ConnectionMode::WaitingForServerResponse) => {
                
                Connection(stream, keep_alive, event, ConnectionMode::WaitingForServerResponse)
            }
			
            Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(str, done))  => {
                
				transform_from_sending_to_user(events, stream, keep_alive, event, str, done)
            },
			
			Connection(stream, keep_alive, event, ConnectionMode::Close)  => {
				
				Connection(stream, keep_alive, event, ConnectionMode::Close)
			}
        }
	}
}


fn transform_from_waiting_for_user(events: EventSet, mut stream: TcpStream, keep_alive: bool, event: Event, mut buf: [u8; 2048], done: usize) -> Connection {
    
    if events.is_readable() {
        
        let total = buf.len();

        println!("total count {}", &done);

        return match stream.try_read(&mut buf[done..total]) {

            Ok(Some(size)) => {

                if size > 0 {
                    
                    let done = done + size;
                    
                    println!("read : {}", size);
                    
                    
                    let mut headers = [httparse::EMPTY_HEADER; 100];
                    let mut req     = httparse::Request::new(&mut headers);

                    match req.parse(&buf) {

                        Ok(httparse::Status::Complete(size_parse)) => {
                            
                            println!("parse ok, get count {}, parse count {}", done, size_parse);
                            
                            //let request = Request::new(req);
                            
                            match Request::new(req) {
                                
                                Ok(request) => {
                                    
                                    println!("Request::new ok");
                                    
                                    
                                    //TODO - get info about keep alive
                                    
                                    
                                    Connection(stream, keep_alive, event, ConnectionMode::ParsedRequest(request))
                                }
                                
                                Err(mess) => {
                                    
//TODO - błąd 400
//trzeba też zamknąć połączenie

                                    Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done))
                                }
                            }
                        }

                                                            //częściowe parsowanie
                        Ok(httparse::Status::Partial) => {
                            
                            Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done))
                        }

                        Err(err) => {

//TODO - 400 error http
//zamknij połączenie

                            match err {
                                httparse::Error::HeaderName => {
                                    println!("header name");
                                }
                                _ => {
                                    println!("error parse {:?}", err);
                                }
                            }
                            
                            /* HeaderName, HeaderValue, NewLine, Status, Token, TooManyHeaders, Version */
                            
                            Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done))
                        }
                    }
                    
                    
                    //uruchom parser
                        //jeśli się udało sparsować, to git

                    //jeśli osiągneliśmy całkowity rozmiar bufora a mimo to nie udało się sparsować danych
                        //to rzuć błędem że nieprawidłowe zapytanie
                
                } else {
                    
                    Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done))
                }
            }

            Ok(None) => {
                println!("no data");
                Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done))
            }

            Err(err) => {
                println!("error read from socket {:?}", err);
                Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done))
            }
        }



        //czytaj, odczytane dane przekaż do parsera
        //jeśli otrzymalismy poprawny obiekt requestu to :
            // przełącz stan tego obiektu połączenia, na oczekiwanie na dane z serwera
            // wyślij kanałem odpowiednią informację o requescie
            // zwróć informację na zewnątrz tej funkcji że nic się nie dzieje z tym połaczeniem
    
    } else {

        //trzeba też ustawić jakiś timeout czekania na dane od użytkownika

        return Connection(stream, keep_alive, event, ConnectionMode::ReadingRequest(buf, done));
    }
}


fn transform_from_sending_to_user(events: EventSet, mut stream: TcpStream, keep_alive: bool, event: Event, str: Vec<u8>, done: usize) -> Connection{

    if events.is_writable() {

        return match stream.try_write(&str[done..str.len()]) {

            Ok(Some(size)) => {

                println!("write data count='{}'", size);

                if size > 0 {

                    let done = done + size;

                                                    //send all data to browser
                    if done == str.len() {
                        
                        /*
                        if keep_alive == true {

                            //keep connection
                            //TODO

                        } else {
                            */
                                //close connection

                            return Connection(stream, keep_alive, event, ConnectionMode::Close);
                        //}
                    
                    } else if done < str.len() {

                        return Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(str, done));
                        
                    } else {
                        
                        unreachable!();
                    }
                
                } else {
                    
                    return Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(str, done));
                }
            }
            
            Ok(None) => {
                
                println!("empty write");
                Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(str, done))
            }
            
            Err(err) => {
                
                println!("error write to socket {:?}", err);
                Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(str, done))
            }
        }
    
    } else {
        
        Connection(stream, keep_alive, event, ConnectionMode::SendingResponse(str, done))
    }
}
