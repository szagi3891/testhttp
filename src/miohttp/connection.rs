use mio::{EventLoop, Token, EventSet, TryRead, TryWrite};
use mio::tcp::{TcpStream};
use httparse;
use miohttp::server::{Event, MyHandler};
use miohttp::request::{PreRequest, Request};
use miohttp::response;



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


//socket, keep alive, actuall event set, current mode
//pub struct Connection (TcpStream, bool, ConnectionMode);

pub struct Connection {
    pub stream: TcpStream,
    mode: ConnectionMode,
}


//TODO - może warto z połączeniem przechowywać również token ... ?

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

    pub fn send_data_to_user(self, tok: Token, response: response::Response) -> Connection {

        println!("transformuję połączenie -> send_data_to_user");

        let new_connection = match self.mode {

            ConnectionMode::WaitingForServerResponse(keep_alive) => {
                
                self.replace_mode(ConnectionMode::SendingResponse(keep_alive, response.as_bytes(), 0))
            }

            _ => {

                println!("TODO - ustawić strumień błędów i wrzucić odpowiedni komunikat : {}", tok.as_usize());
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
    
    pub fn ready(self, events: EventSet, token: &Token, event_loop: &mut EventLoop<MyHandler>) -> (Connection, Option<Request>) {
        
        if events.is_error() {
            println!("EVENT ERROR {}", token.as_usize());
            panic!("TODO");
        }

        if events.is_hup() {
            println!("EVENT HUP - close - {} {:?}", token.as_usize(), events);
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

                (transform_from_sending_to_user(self.stream, keep_alive, events, str, done), None)
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
                            
                            match PreRequest::new(req) {

                                Ok(pre_request) => {
                                    
                                    let request = pre_request.bind(&token, event_loop.channel());
                                    
                                    println!("Request::new ok");

                                    let keep_alive = request.is_header_set("Connection", "keep-alive");

                                    (Connection::make(stream, ConnectionMode::WaitingForServerResponse(keep_alive)), Some(request))
                                }

                                Err(mess) => {

//TODO - błąd 400
//trzeba też zamknąć połączenie

                                    (Connection::make(stream, ConnectionMode::ReadingRequest(buf, done)), None)
                                }
                            }
                        }

                                                            //częściowe parsowanie
                        Ok(httparse::Status::Partial) => {

                            (Connection::make(stream, ConnectionMode::ReadingRequest(buf, done)), None)
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

                            (Connection::make(stream, (ConnectionMode::ReadingRequest(buf, done))), None)
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
                println!("no data");
                (Connection::make(stream, (ConnectionMode::ReadingRequest(buf, done))), None)
            }

            Err(err) => {
                println!("error read from socket {:?}", err);
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


fn transform_from_sending_to_user(mut stream: TcpStream, keep_alive: bool, events: EventSet, str: Vec<u8>, done: usize) -> Connection {

    if events.is_writable() {

        return match stream.try_write(&str[done..str.len()]) {

            Ok(Some(size)) => {

                println!("write data count='{}'", size);

                if size > 0 {

                    let done = done + size;

                                                    //send all data to browser
                    if done == str.len() {

                                                    //eep connection
                        if keep_alive == true {

                            println!("PODTRZYMUJĘ POŁĄCZENIE !!");
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

                println!("empty write");
                Connection::make(stream, ConnectionMode::SendingResponse(keep_alive, str, done))
            }

            Err(err) => {

                println!("error write to socket {:?}", err);
                Connection::make(stream, ConnectionMode::SendingResponse(keep_alive, str, done))
            }
        }

    } else {

        Connection::make(stream, ConnectionMode::SendingResponse(keep_alive, str, done))
    }
}
