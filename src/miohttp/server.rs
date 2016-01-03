use mio::{Token, EventLoop, EventSet, PollOpt, Handler, Timeout};
use mio::tcp::{TcpListener};
//use mio::util::Slab;                              //TODO - użyć tego modułu zamiast hashmapy
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use miohttp::connection::{Connection, TimerMode};
use miohttp::token_gen::TokenGen;
use miohttp::request;
use miohttp::response;
use miohttp::log;

// Define a handler to process the events
pub struct MyHandler {
    token           : Token,
    server          : TcpListener,
    hash            : HashMap<Token, (Connection, Event, Option<Timeout>)>,
    tokens          : TokenGen,
    send            : mpsc::Sender<request::Request>,
    timeout_reading : u64,
    timeout_writing : u64,
}


                                //typ event who is set for socket in event_loop
//#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[derive(PartialEq)]
pub enum Event {
    Init,
    Write,
    Read,
    None
}


impl Handler for MyHandler {

    type Timeout = Token;
    type Message = (Token, response::Response);

    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {

        log::info(format!("miohttp {} -> ready, {:?} (is server = {})", token.as_usize(), events, token == self.token));

        if token == self.token {
            self.new_connection(event_loop);
        } else {
            self.socket_ready(event_loop, &token, events);
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {

        match msg {
            (token, response) => {
                self.send_data_to_user(event_loop, token, response);
            }
        };
    }

    fn timeout(&mut self, _: &mut EventLoop<Self>, token: Self::Timeout) {
        
        self.timeout_trigger(&token);
    }
}


impl MyHandler {

    pub fn new(ip: &String, timeout_reading: u64, timeout_writing:u64, tx: mpsc::Sender<request::Request>) {

        let mut tokens = TokenGen::new();

        let mut event_loop = EventLoop::new().unwrap();

        let addr = ip.parse().unwrap();

        let server = TcpListener::bind(&addr).unwrap();

        let token = tokens.get();

        event_loop.register(&server, token, EventSet::readable(), PollOpt::edge()).unwrap();

        let mut inst = MyHandler{
            token           : token,
            server          : server,
            hash            : HashMap::new(),
            tokens          : tokens,
            send            : tx,
            timeout_reading : timeout_reading,
            timeout_writing : timeout_writing,
        };

        thread::spawn(move || {

            event_loop.run(&mut inst).unwrap();
        });
    }

    fn send_data_to_user(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, response: response::Response) {

        match self.get_connection(&token) {
            
            Some((connection, old_event, timeout)) => {

                let new_connection = connection.send_data_to_user(token.clone(), response);

                self.insert_connection(&token, new_connection, old_event, timeout, event_loop);
            }

            None => {
                
                log::info(format!("miohttp {} -> send_data_to_user: no socket", token.as_usize()));
            }
        }
    }
    
    
    fn timeout_trigger(&mut self, token: &Token) {
        
        match self.get_connection(&token) {

            Some((_, _, _)) => {
                
                log::info(format!("miohttp {} -> timeout_trigger ok", token.as_usize()));
            }

            None => {
                
                log::error(format!("miohttp {} -> timeout_trigger error", token.as_usize()));
            }
        }
    }
    
    
    fn new_connection(&mut self, event_loop: &mut EventLoop<MyHandler>) {
        
        loop {
            match self.server.accept() {

                Ok(Some((stream, addr))) => {

                    let token = self.tokens.get();
                    
                    log::info(format!("miohttp {} -> new connection, addr = {}", token.as_usize(), addr));

                    let connection = Connection::new(stream);

                    self.insert_connection(&token, connection, Event::Init, None, event_loop);
                }

                Ok(None) => {
                    return;
                }

                Err(err) => {
                    
                    log::error(format!("miohttp {} -> new connection err {}", self.token.as_usize(), err));
                    return;
                }
            };
        }
    }

    fn socket_ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: &Token, events: EventSet) {
        
        match self.get_connection(&token) {

            Some((connection, old_event, timeout)) => {
                
                let (new_connection, request_opt) = connection.ready(events, token, event_loop);
                
                if new_connection.in_state_close() {
                    
                                                //TODO - trzeba to bardziej elegancko rozwiązać
                    match timeout {
                        Some(timeout) => {
                            let _ = event_loop.clear_timeout(timeout);
                        }
                        None => {}
                    }
                    
                    return;
                }
                
                match request_opt {
                    
                    Some(request) => {
                        
                        let _ = self.send.send(request);
                    }

                    None => {}
                }

                self.insert_connection(&token, new_connection, old_event, timeout, event_loop);
            }

            None => {
                
                log::info(format!("miohttp {} -> socket ready: no socket by token", token.as_usize()));
            }
        };
    }


    fn set_event(&mut self, connection: &Connection, token: &Token, old_event: &Event, new_event: &Event, event_loop: &mut EventLoop<MyHandler>) -> String {

        let pool_opt    = PollOpt::edge() | PollOpt::oneshot();

        let event_none  = EventSet::error() | EventSet::hup();
        let event_write = event_none | EventSet::writable();
        let event_read  = event_none | EventSet::readable();

        if *old_event == Event::Init {

            match *new_event {
                Event::Init => {
                    format!("register: none")
                },
                Event::Write => {
                    event_loop.register(&connection.stream, token.clone(), event_write, pool_opt).unwrap();
                    format!("register: {:?}", event_write)
                },
                Event::Read => {
                    event_loop.register(&connection.stream, token.clone(), event_read, pool_opt).unwrap();
                    format!("register: {:?}", event_read)
                },
                Event::None => {
                    event_loop.register(&connection.stream, token.clone(), event_none, pool_opt).unwrap();
                    format!("register: {:?}", event_none)
                }
            }

        } else {

            match *new_event {
                Event::Init => {
                    format!("reregister: none")
                },
                Event::Write => {
                    event_loop.reregister(&connection.stream, token.clone(), event_write, pool_opt).unwrap();
                    format!("reregister: {:?}", event_write)
                },
                Event::Read => {
                    event_loop.reregister(&connection.stream, token.clone(), event_read, pool_opt).unwrap();
                    format!("reregister: {:?}", event_read)
                },
                Event::None => {
                    event_loop.reregister(&connection.stream, token.clone(), event_none, pool_opt).unwrap();
                    format!("reregister: {:?}", event_none)
                }
            }
        }
    }

    
    fn set_timer(&mut self, token: &Token, timeout: Option<Timeout>, timer_mode: TimerMode, event_loop: &mut EventLoop<MyHandler>) -> (Option<Timeout>, String) {
        
        match timeout {
            
            Some(timeout) => {
                
                match timer_mode {
                    
                    TimerMode::In  => (Some(timeout), "keep".to_string()),
                    TimerMode::Out => (Some(timeout), "keep".to_string()),
                    
                    TimerMode::None => {
                        let _ = event_loop.clear_timeout(timeout);
                        (None, "clear".to_string())
                    },
                }
            },
            
            None => {
                
                match timer_mode {
                    
                    TimerMode::In => {
                        
                        match event_loop.timeout_ms(token.clone(), self.timeout_reading) {
                            
                            Ok(timeout) => (Some(timeout), "timer in set".to_string()),
                            Err(err)    => (None , format!("timer in error {:?}", err)),
                        }
                            
                    },
                    
                    TimerMode::Out => {
                        
                        match event_loop.timeout_ms(token.clone(), self.timeout_writing) {
                            
                            Ok(timeout) => (Some(timeout), "timer out set".to_string()),
                            Err(err)    => (None , format!("timer out error {:?}", err)),
                        }
                    },
                    
                    TimerMode::None => (None, "none".to_string()),
                }
            },
        }
    }
    
    fn insert_connection(&mut self, token: &Token, connection: Connection, old_event: Event, timeout: Option<Timeout>, event_loop: &mut EventLoop<MyHandler>) {

        let new_event = connection.get_event();
        
        let mess_event = if old_event != new_event {
            self.set_event(&connection, token, &old_event, &new_event, event_loop)
        } else {
            "none".to_string()
        };
        
        
        let (new_timer, timer_message) = self.set_timer(token, timeout, connection.get_timer_mode(), event_loop);
        
        
        log::info(format!("miohttp {} -> set mode {}, {}, timer {}", token.as_usize(), connection.get_name(), mess_event, timer_message));
        
        self.hash.insert(token.clone(), (connection, new_event, new_timer));
        
        println!("count hasmapy after insert {}", self.hash.len());
    }
    
    fn get_connection(&mut self, token: &Token) -> Option<(Connection, Event, Option<Timeout>)> {

        let res = self.hash.remove(&token);
        
        println!("hashmap after decrement {}", self.hash.len());
        
        res
    }


}

