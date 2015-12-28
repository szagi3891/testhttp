use mio::{Token, EventLoop, EventSet, PollOpt, Handler};
use mio;
use mio::tcp::{TcpListener};
//use mio::util::Slab;                              //TODO - użyć tego modułu zamiast hashmapy
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use miohttp::connection::{Connection};
use miohttp::token_gen::TokenGen;
use miohttp::request;
use miohttp::response;
use miohttp::log;


// Define a handler to process the events
pub struct MyHandler {
    token    : Token,
    server   : TcpListener,
    hash     : HashMap<Token, Connection>,
    tokens   : TokenGen,
	send     : mpsc::Sender<(request::Request, Token, mio::Sender<(Token, response::Response)>)>,
}


impl Handler for MyHandler {

    type Timeout = ();
    type Message = (Token, response::Response);

    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
		
        //log::error("testowy komunikat");
		
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
		
		match self.hash.remove(&token) {
			
            Some(connection) => {
				
				let new_connection = connection.send_data_to_user(event_loop, token.clone(), response);
				
				self.hash.insert(token.clone(), new_connection);
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

                    let tok = self.tokens.get();
                    
                    println!("new connection ok - {} {:?}", addr, &tok);
                    
                    let connection = Connection::new(stream, tok.clone(), event_loop);

                    self.hash.insert(tok, connection);
                    
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
                        
						self.send.send((request, token.clone(), event_loop.channel()));
						
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

}


