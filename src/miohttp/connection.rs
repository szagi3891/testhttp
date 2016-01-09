use mio::{EventLoop, Token, EventSet, TryRead, TryWrite};
use mio::tcp::{TcpStream};
use httparse;

use miohttp::server::{Event, MyHandler};
use miohttp::request::{PreRequest, Request};
use miohttp::response;
use log;


enum ConnectionMode {

                                                    //czytanie requestu
    ReadingRequest([u8; 2048], usize),
                                                    //oczekiwanie na wygenerowanie odpowiedzi serwera (bool to keep alive)
    WaitingForServerResponse(bool),
                                                    //wysyłanie odpowiedz (bool to keep alive)
    SendingResponse(bool, Vec<u8>, usize),
                                                    //połączenie do zamknięcia
    Close,
}


pub enum TimerMode {
    In,
    Out,
    None,
}


pub struct Connection {
    pub stream: TcpStream,
    mode: ConnectionMode,
}


impl Connection {


    pub fn new(stream: TcpStream) -> Connection {

        Connection {
            stream : stream,
            mode   : ConnectionMode::ReadingRequest([0u8; 2048], 0),
        }
    }

    fn make(stream: TcpStream, mode: ConnectionMode) -> Connection {

        Connection {
            stream : stream,
            mode   : mode,
        }
    }

    fn replace_mode(self, mode: ConnectionMode) -> Connection {
        
        Connection {
            stream : self.stream,
            mode   : mode,
        }
    }

    pub fn in_state_close(&self) -> bool {

        match self.mode {
            ConnectionMode::Close => true,
            _ => false
        }
    }

    pub fn send_data_to_user(self, token: Token, response: response::Response) -> Connection {
        
        let new_connection = match self.mode {

            ConnectionMode::WaitingForServerResponse(keep_alive) => {
                
                self.replace_mode(ConnectionMode::SendingResponse(keep_alive, response.as_bytes(), 0))
            }

            _ => {
                
                log::error(format!("miohttp {} -> send_data_to_user: incorect state", token.as_usize()));
                self
            }
        };

        new_connection
    }

    pub fn get_event(&self) -> Event {

        match self.mode {
            
            ConnectionMode::ReadingRequest(_, _)        => Event::Read,
            ConnectionMode::WaitingForServerResponse(_) => Event::None,
            ConnectionMode::SendingResponse(_, _, _)    => Event::Write,
            ConnectionMode::Close                       => Event::None
        }
    }
    
    pub fn get_timer_mode(&self) -> TimerMode {
        
        match self.mode {
            
            ConnectionMode::ReadingRequest(_, _)        => TimerMode::In,
            ConnectionMode::WaitingForServerResponse(_) => TimerMode::None,
            ConnectionMode::SendingResponse(_, _, _)    => TimerMode::Out,
            ConnectionMode::Close                       => TimerMode::None
        }
    }
    
    pub fn get_name(&self) -> &str {
        
        match self.mode {
            
            ConnectionMode::ReadingRequest(_, _)        => "ReadingRequest",
            ConnectionMode::WaitingForServerResponse(_) => "WaitingForServerResponse",
            ConnectionMode::SendingResponse(_, _, _)    => "SendingResponse",
            ConnectionMode::Close                       => "Close"
        }
    }
    
    pub fn ready(self, events: EventSet, token: &Token, event_loop: &mut EventLoop<MyHandler>) -> (Connection, Option<Request>) {
        
        if events.is_error() {
            
            log::error(format!("miohttp {} -> ready error, {:?}", token.as_usize(), events));
            return (self.replace_mode(ConnectionMode::Close), None);
        }
        
        if events.is_hup() {
            
            log::info(format!("miohttp {} -> ready, event hup, {:?}", token.as_usize(), events));
            return (self.replace_mode(ConnectionMode::Close), None);
        }
        
        self.transform(events, event_loop, token)
    }



    fn transform(self, events: EventSet, event_loop: &mut EventLoop<MyHandler>, token: &Token) -> (Connection, Option<Request>) {

        match self.mode {

            ConnectionMode::ReadingRequest(buf, done) => {

                transform_from_waiting_for_user(self.stream, events, buf, done, event_loop, token)
            }

            ConnectionMode::WaitingForServerResponse(keep_alive) => {

                (Connection::make(self.stream, ConnectionMode::WaitingForServerResponse(keep_alive)), None)
            }
            
            ConnectionMode::SendingResponse(keep_alive, str, done) => {

                (transform_from_sending_to_user(self.stream, token, keep_alive, events, str, done), None)
            },

            ConnectionMode::Close => {

                (Connection::make(self.stream, ConnectionMode::Close), None)
            }
        }
    }
}

fn transform_from_waiting_for_user(mut stream: TcpStream, events: EventSet, mut buf: [u8; 2048], done: usize, event_loop: &mut EventLoop<MyHandler>, token: &Token) -> (Connection, Option<Request>) {

    if events.is_readable() {

        let total = buf.len();
        
        return match stream.try_read(&mut buf[done..total]) {

            Ok(Some(size)) => {

                if size > 0 {

                    let done = done + size;
                    
                    let mut headers = [httparse::EMPTY_HEADER; 100];
                    let mut req     = httparse::Request::new(&mut headers);

                    match req.parse(&buf) {

                        Ok(httparse::Status::Complete(_)) => {      /*size_parse*/
                            
                            match PreRequest::new(req) {

                                Ok(pre_request) => {
                                    
                                    let request = pre_request.bind(&token, event_loop.channel());
                                    
                                    let keep_alive = request.is_header_set("Connection", "keep-alive");

                                    (Connection::make(stream, ConnectionMode::WaitingForServerResponse(keep_alive)), Some(request))
                                }

                                Err(err) => {
                                    
                                    log::error(format!("miohttp {} -> error prepare request, {:?}", token.as_usize(), err));
                                    
                                    let response_400 = response::Response::create_400();
                                    (Connection::make(stream, ConnectionMode::SendingResponse(false, response_400.as_bytes(), 0)), None)
                                }
                            }
                        }

                                                            //częściowe parsowanie
                        Ok(httparse::Status::Partial) => {

                            (Connection::make(stream, ConnectionMode::ReadingRequest(buf, done)), None)
                        }

                        Err(err) => {
                            
                            match err {
                                
//TODO - zrobić formatowanie komunikatu z błędem -> wrzucać na strumień błędów
                                
                                httparse::Error::HeaderName => {
                                    println!("header name");
                                }
                                _ => {
                                    println!("error parse {:?}", err);
                                }
                            }

                            /* HeaderName, HeaderValue, NewLine, Status, Token, TooManyHeaders, Version */
                            
                            let response_400 = response::Response::create_400();
                            (Connection::make(stream, ConnectionMode::SendingResponse(false, response_400.as_bytes(), 0)), None)
                        }
                    }


                    //uruchom parser
                        //jeśli się udało sparsować, to git

                    //jeśli osiągneliśmy całkowity rozmiar bufora a mimo to nie udało się sparsować danych
                        //to rzuć błędem że nieprawidłowe zapytanie

                } else {

                    (Connection::make(stream, (ConnectionMode::ReadingRequest(buf, done))), None)
                }
            }

            Ok(None) => {
                (Connection::make(stream, (ConnectionMode::ReadingRequest(buf, done))), None)
            }

            Err(err) => {
                
                log::error(format!("miohttp {} -> error read from socket, {:?}", token.as_usize(), err));
                
                (Connection::make(stream, (ConnectionMode::ReadingRequest(buf, done))), None)
            }
        }



        //czytaj, odczytane dane przekaż do parsera
        //jeśli otrzymalismy poprawny obiekt requestu to :
            // przełącz stan tego obiektu połączenia, na oczekiwanie na dane z serwera
            // wyślij kanałem odpowiednią informację o requescie
            // zwróć informację na zewnątrz tej funkcji że nic się nie dzieje z tym połaczeniem

    } else {

        //trzeba też ustawić jakiś timeout czekania na dane od użytkownika

        return (Connection::make(stream, ConnectionMode::ReadingRequest(buf, done)), None);
    }
}


fn transform_from_sending_to_user(mut stream: TcpStream, token: &Token, keep_alive: bool, events: EventSet, str: Vec<u8>, done: usize) -> Connection {

    if events.is_writable() {

        return match stream.try_write(&str[done..str.len()]) {

            Ok(Some(size)) => {
                
                if size > 0 {

                    let done = done + size;

                                                    //send all data to browser
                    if done == str.len() {

                                                    //eep connection
                        if keep_alive == true {

                            log::debug(format!("miohttp {} -> keep alive", token.as_usize()));
                            
                            return Connection::make(stream, (ConnectionMode::ReadingRequest([0u8; 2048], 0)));

                                                    //close connection
                        } else {
                            return Connection::make(stream, (ConnectionMode::Close));
                        }

                    } else if done < str.len() {

                        return Connection::make(stream, (ConnectionMode::SendingResponse(keep_alive, str, done)));

                    } else {

                        unreachable!();
                    }

                } else {

                    return Connection::make(stream, ConnectionMode::SendingResponse(keep_alive, str, done));
                }
            }

            Ok(None) => {

                Connection::make(stream, ConnectionMode::SendingResponse(keep_alive, str, done))
            }

            Err(err) => {

                log::error(format!("miohttp {} -> error write to socket, {:?}", token.as_usize(), err));
                Connection::make(stream, ConnectionMode::SendingResponse(keep_alive, str, done))
            }
        }

    } else {

        Connection::make(stream, ConnectionMode::SendingResponse(keep_alive, str, done))
    }
}
