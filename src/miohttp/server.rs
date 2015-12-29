use mio::{Token, EventLoop, EventSet, PollOpt, Handler};
use mio;
use mio::tcp::{TcpListener};
//use mio::util::Slab;                              //TODO - użyć tego modułu zamiast hashmapy
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
//use std::sync::mpsc::{Sender};
use miohttp::connection::{Connection};
use miohttp::token_gen::TokenGen;
use miohttp::request;
use miohttp::response;
use miohttp::log;
//use miohttp::hashmap_connection;


// Define a handler to process the events
pub struct MyHandler {
    token    : Token,
    server   : TcpListener,
	hash     : HashMap<Token, (Connection, Event)>,
    //hash     : hashmap_connection::Hashmap,
    tokens   : TokenGen,
	send     : mpsc::Sender<(request::Request, Token, mio::Sender<(Token, response::Response)>)>,
}


                                //typ event who is set for socket in event_loop
enum Event {
    Init,
    Write,
    Read,
    None
}


impl Handler for MyHandler {

    type Timeout = Token;
    type Message = (Token, response::Response);
	
    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
		
        println!(">>>>>>>>>>> {:?} {:?} (is server = {})", token, events, token == self.token);
        
        if token == self.token {
            self.new_connection(event_loop);
        } else {
            self.socket_ready(event_loop, token, events);
        }
    }
	
    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {
		
		match msg {
			(token, response) => {
				self.send_data_to_user(event_loop, token, response);
			}
		};
    }
	
	fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: Self::Timeout) {
		
		println!("timeout zaszedł {:?}", timeout);
    }
}


impl MyHandler {

    pub fn new(ip: &String, tx: mpsc::Sender<(request::Request, Token, mio::Sender<(Token, response::Response)>)>) {

        let mut tokens = TokenGen::new();

        let mut event_loop = EventLoop::new().unwrap();

        let addr = ip.parse().unwrap();

        let server = TcpListener::bind(&addr).unwrap();

        let token = tokens.get();
        
        event_loop.register(&server, token, EventSet::readable(), PollOpt::edge()).unwrap();
		
        let mut inst = MyHandler{
			token  : token,
			server : server,
			hash   : HashMap::new(),
			tokens : tokens,
			send   : tx,
		};
		
		thread::spawn(move || {
			
        	event_loop.run(&mut inst).unwrap();
		});
    }
	
	fn send_data_to_user(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, response: response::Response) {
		
		println!("odebrano kominikat z kanału {} {:?}", token.as_usize(), response);
		
		match self.get_connection(&token) {
			
			Some((connection, old_event)) => {
				
				let new_connection = connection.send_data_to_user(event_loop, token.clone(), response);
				
				self.insert_connection(&token, new_connection, old_event);
			}
			
			None => {
				println!("socket_ready: no socket by token: {:?}", &token);
			}
		}
	}
	
    fn new_connection(&mut self, event_loop: &mut EventLoop<MyHandler>) {

        println!("new connection - prepending");

        loop {
            match self.server.accept() {

                Ok(Some((stream, addr))) => {
					
                    let token = self.tokens.get();
                    
                    println!("new connection ok - {} {:?}", addr, &token);
                    
                    let connection = Connection::new(stream, token.clone(), event_loop);

                    //self.hash.insert(tok, connection);
					self.insert_connection(&token, connection, Event::Init);
                    
                    println!("hashmap after new connection {}", self.hash.len());
                }

                Ok(None) => {
                    
                    println!("no new connection");
                    return;
                }

                Err(e) => {
                    
                    println!("error accept mew connection: {}", e);
                    return;
                }
            };
        }
    }
	
    fn socket_ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {

        println!("count hasmapy before socket_ready {}", self.hash.len());
        
        match self.hash.remove(&token) {
			
            Some(connection) => {
				
                let new_connection = connection.ready(events, token.clone(), event_loop);
				
                if new_connection.in_state_close() {
                    
                    println!("!!!!!!!!!!!!!! server close connection {:?} !!!!!!!!!!!!!!", &token);
                    println!("count hasmapy after ready after close {}", self.hash.len());
                    println!("\n\n\n");

                    return;
                }
                
                
                let (new_connection, request_opt) = new_connection.get_request(token.clone(), event_loop);
                
                match request_opt {
                    
                    Some(request) => {
                        
						let _ = self.send.send((request, token.clone(), event_loop.channel()));
						
                        //println!("request to send: {:?}", request);
                        //TODO, wyślij go przez kanał do zainteresowanych, self.send.send(request)
                    }
                    
                    None => {}
                }
                
				
                
				self.hash.insert(token.clone(), new_connection);
            }
			
            None => {
				
                println!("socket_ready: no socket by token: {:?}", &token);
            }
        };
		
		
		println!("count hasmapy after ready {}", self.hash.len());
		
    }

	fn insert_connection(&self, &token: Token, connection: Connection, old_event: Event) {
		
		//tutaj musimy wyznaczyć nowe eventy
		//na podstawie zmiany, trzeba dokonać odpowiednich rejestracji, rerejestracji
		
		self.hash.insert(token.clone(), (connection, Event::Init));
	}
	
	fn get_connection(&self, &token: Token) -> Option<(Connection, Event)> {
		
		self.hash.remove(&token)
		//Some((connection, event)) => {
		//}
	}
	
	
	/*
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
	*/
}


