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

        println!(">>>>>>>>>>> {:?} {:?} (is server = {})", token, events, token == self.token);

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
                println!("socket_ready: no socket by token: {:?}", &token);
            }
        }
    }
    
    
    fn timeout_trigger(&mut self, token: &Token) {
        
        match self.get_connection(&token) {

            Some((_, _, _)) => {
                
                println!("timeout - poprawnie zamknięte połączenie {:?}", token);
            }

            None => {
                println!("TODO - error, brak takiego połączenia, wrzucić loga w strumień błędów {:?}", token);
            }
        }
    }
    
    
    fn new_connection(&mut self, event_loop: &mut EventLoop<MyHandler>) {
        
        loop {
            match self.server.accept() {

                Ok(Some((stream, addr))) => {

                    let token = self.tokens.get();

                    println!("new connection ok - {} {:?}", addr, &token);

                    let connection = Connection::new(stream);

                    self.insert_connection(&token, connection, Event::Init, None, event_loop);
                }

                Ok(None) => {
                    return;
                }

                Err(e) => {

                    println!("error accept mew connection: {}", e);
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

                println!("socket_ready: no socket by token: {:?}", &token);
            }
        };
    }


    fn set_event(&mut self, connection: &Connection, token: &Token, old_event: &Event, new_event: &Event, event_loop: &mut EventLoop<MyHandler>) {

        let pool_opt    = PollOpt::edge() | PollOpt::oneshot();

        let event_none  = EventSet::error() | EventSet::hup();
        let event_write = event_none | EventSet::writable();
        let event_read  = event_none | EventSet::readable();

        if *old_event == Event::Init {

            match *new_event {
                Event::Init => {},
                Event::Write => {
                    println!("----------> register: {:?} {:?}", token, event_write);
                    event_loop.register(&connection.stream, token.clone(), event_write, pool_opt).unwrap();
                },
                Event::Read => {
                    println!("----------> register: {:?} {:?}", token, event_read);
                    event_loop.register(&connection.stream, token.clone(), event_read, pool_opt).unwrap();
                },
                Event::None => {
                    println!("----------> register: {:?} {:?}", token, event_none);
                    event_loop.register(&connection.stream, token.clone(), event_none, pool_opt).unwrap();
                }
            }

        } else {

            match *new_event {
                Event::Init => {},
                Event::Write => {
                    println!("----------> reregister: {:?} {:?}", token, event_write);
                    event_loop.reregister(&connection.stream, token.clone(), event_write, pool_opt).unwrap();
                },
                Event::Read => {
                    println!("----------> reregister: {:?} {:?}", token, event_read);
                    event_loop.reregister(&connection.stream, token.clone(), event_read, pool_opt).unwrap();
                },
                Event::None => {
                    println!("----------> reregister: {:?} {:?}", token, event_none);
                    event_loop.reregister(&connection.stream, token.clone(), event_none, pool_opt).unwrap();
                }
            }
        }
    }

    
    fn set_timer(&mut self, token: &Token, timeout: Option<Timeout>, timer_mode: TimerMode, event_loop: &mut EventLoop<MyHandler>) -> Option<Timeout> {
        
        match timeout {
            
            Some(timeout) => {
                
                match timer_mode {
                    
                    TimerMode::In => {
                        Some(timeout)
                    },
                    
                    TimerMode::Out => {
                        Some(timeout)
                    },
                    
                    TimerMode::None => {
                        
                        println!("ZERUJĘ TIMER {:?}", token);
                        
                        let _ = event_loop.clear_timeout(timeout);
                        None
                    },
                }
            },
            
            None => {
                
                match timer_mode {
                    
                    TimerMode::In => {
                        
                        println!("USTAWIAM TIMER IN {:?}", token);
                        
                        match event_loop.timeout_ms(token.clone(), self.timeout_reading) {
                            
                            Ok(timeout) => {
                                
                                println!("USTAWIAM TIMER IN - udane");
                                Some(timeout)
                            },
                            
                            Err(err) => {
                                
                                //TODO - błąd wrzucić w logowanie na strumień błędów
                                
                                println!("USTAWIAM TIMER IN - nieudane");
                                None
                            }
                        }
                            
                    },
                    
                    TimerMode::Out => {
                        
                        println!("USTAWIAM TIMER OUT {:?}", token);
                        
                        match event_loop.timeout_ms(token.clone(), self.timeout_writing) {
                            
                            Ok(timeout) => {
                                
                                println!("USTAWIAM TIMER OUT - udane");
                                Some(timeout)
                            },
                            
                            Err(err) => {
                                
                                //TODO - błąd wrzucić w logowanie na strumień błędów
                                
                                println!("USTAWIAM TIMER OUT - nieudane");
                                None
                            }
                        }
                    },
                    
                    TimerMode::None => {
                        None
                    },
                }
            },
        }
    }
    
    
    
    
    fn insert_connection(&mut self, token: &Token, connection: Connection, old_event: Event, timeout: Option<Timeout>, event_loop: &mut EventLoop<MyHandler>) {

        let new_event = connection.get_event();
        
        /*
        println!("----------> set mode : WaitingForDataUser");
        println!("----------> set mode : WaitingForDataServer");
        println!("----------> set mode : DataToSendUser");
        println!("----------> set mode : Close");
        */
        
        if old_event != new_event {
            self.set_event(&connection, token, &old_event, &new_event, event_loop);
        }
        
        
        let new_timer = self.set_timer(token, timeout, connection.get_timer_mode(), event_loop);
        
        
        self.hash.insert(token.clone(), (connection, new_event, new_timer));
        
        
        println!("count hasmapy after insert {}", self.hash.len());
    }
    
    fn get_connection(&mut self, token: &Token) -> Option<(Connection, Event, Option<Timeout>)> {

        let res = self.hash.remove(&token);
        
        println!("hashmap after decrement {}", self.hash.len());
        
        res
    }


}

