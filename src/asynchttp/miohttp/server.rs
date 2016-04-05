use std::collections::HashMap;
use std::io;
use mio::{Token, EventLoop, EventSet, PollOpt, Handler, Timeout};
use mio::tcp::{TcpListener};
//use mio::util::Slab;                 //TODO - użyć tego modułu zamiast hashmapy

use asynchttp::miohttp::response;
use asynchttp::log;
use asynchttp::miohttp::connection::{Connection, TimerMode};
use asynchttp::miohttp::token_gen::TokenGen;
use asynchttp::miohttp::request::Request;

use channels_async::{Sender};
use std::time::Duration;


type FnConvert<Out> = Box<Fn(Request) -> Out + Send + Sync + 'static>;


// Define a handler to process the events
pub struct MyHandler<Out> {
    token           : Token,
    server          : TcpListener,                  //TODO option, Some - serwer nasłuchuje, None - jest w trybie wyłączania
    hash            : HashMap<Token, (Connection, Event, Option<Timeout>)>,
    tokens          : TokenGen,
    channel         : Sender<Out>,                  //TODO - trzeba użyć typu generycznego i pozbyć się tej zależności
    timeout_reading : u64,
    timeout_writing : u64,
    convert_request : FnConvert<Out>,
}


// Event type which is set for socket in event_loop
//#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[derive(PartialEq)]
pub enum Event {
    Init,
    Write,
    Read,
    None
}


impl<Out> Handler for MyHandler<Out> {

    type Timeout = Token;
    type Message = (Token, response::Response);

    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {

        log::debug(format!("miohttp {} -> ready, {:?} (is server = {})", token.as_usize(), events, token == self.token));

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


impl<Out> MyHandler<Out> {

    pub fn new(ip: &String, timeout_reading: u64, timeout_writing:u64, tx: Sender<Request>, convert : FnConvert<Out>) -> Result<(), io::Error> {
        
        let mut tokens = TokenGen::new();

        let mut event_loop = EventLoop::new().unwrap();

        let addr = ip.parse().unwrap();

        let server = match TcpListener::bind(&addr) {
            Ok(server) => server,
            Err(err) => {
                log::error(format!("Unable to bind socket {}: {}", addr, err));
                return Err(err);
            }
        };

        let token = tokens.get();
        
        event_loop.register(&server, token, EventSet::readable(), PollOpt::edge()).unwrap();

        let mut inst = MyHandler{
            token           : token,
            server          : server,
            hash            : HashMap::new(),
            tokens          : tokens,
            channel         : tx,
            timeout_reading : timeout_reading,
            timeout_writing : timeout_writing,
            convert_request : convert,
        };
        
        event_loop.run(&mut inst).unwrap();
        
        Ok(())
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
                
                log::debug(format!("miohttp {} -> timeout_trigger ok", token.as_usize()));
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
                    
                    if let Some(ref timeout_value) = timeout {
                        let _ = event_loop.clear_timeout(timeout_value);
                    }
                    
                    return;
                }
                
                match request_opt {
                    
                    Some(request) => {
                        log::debug(format!("Sending request through channel 1"));
                        self.channel.send(request).unwrap();
                        log::debug(format!("Sending request through channel 2"));
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


    fn set_event(&mut self, connection: &Connection, token: &Token, old_event: &Event, new_event: &Event, event_loop: &mut EventLoop<MyHandler>) -> Result<String, io::Error> {
        
        let pool_opt = PollOpt::edge() | PollOpt::oneshot();
        
        let new_mode = match *new_event {
            Event::Init  => None,
            Event::Write => Some(EventSet::error() | EventSet::hup() | EventSet::writable()),
            Event::Read  => Some(EventSet::error() | EventSet::hup() | EventSet::readable()),
            Event::None  => Some(EventSet::error() | EventSet::hup()),
        };
        
        if *old_event == Event::Init {
        
            match new_mode {
                None => Ok(format!("register: none")),
                Some(mode) => {
                    try!(event_loop.register(&connection.stream, token.clone(), mode, pool_opt));
                    Ok(format!("register: {:?}", mode))
                },
            }
        
        } else {
            
            match new_mode {
                None => Ok(format!("reregister: none")),
                Some(mode) => {
                    try!(event_loop.reregister(&connection.stream, token.clone(), mode, pool_opt));
                    Ok(format!("reregister: {:?}", mode))
                },
            }
        }
    }

    
    fn set_timer(&mut self, token: &Token, timeout: Option<Timeout>, timer_mode: TimerMode, event_loop: &mut EventLoop<MyHandler>) -> (Option<Timeout>, String) {
        
        match timeout {
            
            Some(timeout) => {
                
                match timer_mode {
                    
                    TimerMode::In  => (Some(timeout), "keep".to_owned()),
                    TimerMode::Out => (Some(timeout), "keep".to_owned()),
                    
                    TimerMode::None => {
                        let _ = event_loop.clear_timeout(&timeout);
                        (None, "clear".to_owned())
                    },
                }
            },
            
            None => {
                
                match timer_mode {
                    
                    TimerMode::In => {
                        
                        match event_loop.timeout(token.clone(), Duration::from_millis(self.timeout_reading)) {
                        
                        //match event_loop.timeout_ms(token.clone(), self.timeout_reading) {
                            
                            Ok(timeout) => (Some(timeout), "timer in set".to_owned()),
                            Err(err)    => (None , format!("timer in error {:?}", err)),
                        }
                            
                    },
                    
                    TimerMode::Out => {
                        
                        match event_loop.timeout(token.clone(), Duration::from_millis(self.timeout_writing)) {
                        //match event_loop.timeout_ms(token.clone(), self.timeout_writing) {
                            
                            Ok(timeout) => (Some(timeout), "timer out set".to_owned()),
                            Err(err)    => (None , format!("timer out error {:?}", err)),
                        }
                    },
                    
                    TimerMode::None => (None, "none".to_owned()),
                }
            },
        }
    }
    
    fn insert_connection(&mut self, token: &Token, connection: Connection, old_event: Event, timeout: Option<Timeout>, event_loop: &mut EventLoop<MyHandler>) {

        let new_event = connection.get_event();
        
        let mess_event = if old_event != new_event {
            match self.set_event(&connection, token, &old_event, &new_event, event_loop) {
                Ok(str) => str,
                Err(err) => {
                    log::error(format!("set_event: {}", err));
                    return;
                }
            }
        } else {
            "none".to_owned()
        };
        
        
        let (new_timer, timer_message) = self.set_timer(token, timeout, connection.get_timer_mode(), event_loop);
        
        
        log::debug(format!("miohttp {} -> set mode {}, {}, timer {}", token.as_usize(), connection.get_name(), mess_event, timer_message));
        
        self.hash.insert(token.clone(), (connection, new_event, new_timer));
        
        log::debug(format!("count hasmapy after insert {}", self.hash.len()));
    }
    
    fn get_connection(&mut self, token: &Token) -> Option<(Connection, Event, Option<Timeout>)> {

        let res = self.hash.remove(&token);
        
        log::debug(format!("hashmap after decrement {}", self.hash.len()));
        
        res
    }


}

