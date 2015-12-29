use mio::{Token, EventLoop, EventSet, PollOpt, Handler};
use mio;
use mio::tcp::{TcpListener};
//use mio::util::Slab;                              //TODO - użyć tego modułu zamiast hashmapy
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use miohttp::connection::{Connection};
use miohttp::token_gen::TokenGen;
use miohttp::request;
use miohttp::response;


// Define a handler to process the events
pub struct MyHandler {
    token    : Token,
    server   : TcpListener,
    hash2    : HashMap<Token, (Connection, Event)>,
    tokens   : TokenGen,
    send     : mpsc::Sender<(request::Request, Token, mio::Sender<(Token, response::Response)>)>,
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
            hash2  : HashMap::new(),
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

                let new_connection = connection.send_data_to_user(token.clone(), response);

                self.insert_connection(&token, new_connection, old_event, event_loop);
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

                    let connection = Connection::new(stream);

                    self.insert_connection(&token, connection, Event::Init, event_loop);

                    println!("hashmap after new connection {}", self.connections_count());
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

        println!("count hasmapy before socket_ready {}", self.connections_count());

        match self.get_connection(&token) {

            Some((connection, old_event)) => {

                let (new_connection, request_opt) = connection.ready(events, token.clone());

                if new_connection.in_state_close() {

                    println!("!!!!!!!!!!!!!! server close connection {:?} !!!!!!!!!!!!!!", &token);
                    println!("count hasmapy after ready after close {}", self.connections_count());
                    println!("\n\n\n");

                    return;
                }

                match request_opt {

                    Some(request) => {

                        let _ = self.send.send((request, token.clone(), event_loop.channel()));
                    }

                    None => {}
                }



                self.insert_connection(&token, new_connection, old_event, event_loop);
            }

            None => {

                println!("socket_ready: no socket by token: {:?}", &token);
            }
        };


        println!("count hasmapy after ready {}", self.connections_count());

    }

    fn connections_count(&self) -> usize {
        self.hash2.len()
    }

    /*
    event_loop.register(&stream, token, event_read, pool_opt).unwrap();
    event_loop.register(&stream, token, event_none, pool_opt).unwrap();
    */


    fn set_event(&mut self, connection: &Connection, token: &Token, old_event: &Event, new_event: &Event, event_loop: &mut EventLoop<MyHandler>) /*-> String*/  {

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

    fn insert_connection(&mut self, token: &Token, connection: Connection, old_event: Event, event_loop: &mut EventLoop<MyHandler>) {

        let new_event = connection.get_event();

        /*
        println!("----------> set mode : WaitingForDataUser");
        println!("----------> set mode : ParsedRequest");
        println!("----------> set mode : WaitingForDataServer");
        println!("----------> set mode : DataToSendUser");
        println!("----------> set mode : Close");
        */

        if old_event != new_event {
            self.set_event(&connection, token, &old_event, &new_event, event_loop);
        }

        self.hash2.insert(token.clone(), (connection, new_event));
    }

    fn get_connection(&mut self, token: &Token) -> Option<(Connection, Event)> {

        self.hash2.remove(&token)
    }


}

