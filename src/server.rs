use mio::{Token, EventLoop, EventSet, PollOpt, Handler};
use mio::tcp::{TcpListener};
//use mio::util::Slab;                              //TODO - użyć tego modułu zamiast hashmapy
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::{Sender};
use connection::{Connection};
use token_gen::TokenGen;


// Define a handler to process the events
pub struct MyHandler {
    token    : Token,
    server   : TcpListener,
    hash     : HashMap<Token, Connection>,
    tokens   : TokenGen,
	send     : Sender<String>,
}


impl Handler for MyHandler {

    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
        
        println!(">>>>>>>>>>> {:?} {:?} (is server = {})", token, events, token == self.token);
        
        if token == self.token {

            self.new_connection(event_loop);

        } else {
            self.socket_ready(event_loop, token, events);
        }
    }
}


impl MyHandler {

    pub fn new(ip: &String, tx: Sender<String>) {

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
                    
                    //panic!("zamykam");

                    println!("!!!!!!!!!!!!!! server close connection {:?} !!!!!!!!!!!!!!", &token);
                    println!("count hasmapy after ready after close {}", self.hash.len());
                    println!("\n\n\n");

                    return;
                }
                
                
                //weź obiekt requestu, wyślij go przez kanał do zainteresowanych
                
				
				self.hash.insert(token.clone(), new_connection);
            }
			
            None => {
				
                println!("no socket by token: {:?}", &token);
            }
        };
		
		
		println!("count hasmapy after ready {}", self.hash.len());
		
    }

}


