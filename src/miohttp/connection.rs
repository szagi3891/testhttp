use mio::{Token, EventSet, TryRead, TryWrite};
use mio::tcp::{TcpStream};
use httparse;
use miohttp::server::{Event};
use miohttp::request::Request;
use miohttp::response;



enum ConnectionMode {

                                                    //czytanie requestu
    ReadingRequest([u8; 2048], usize),
                                                    //oczekiwanie na wygenerowanie odpowiedzi serwera
    WaitingForServerResponse,
                                                    //wysyłanie odpowiedz
    SendingResponse(Vec<u8>, usize),
                                                    //połączenie do zamknięcia
    Close,
}



//socket, keep alive, actuall event set, current mode
//pub struct Connection (TcpStream, bool, ConnectionMode);

pub struct Connection {
    pub stream: TcpStream,
    keep_alive: bool,
    mode: ConnectionMode,
}


//TODO - może warto z połączeniem przechowywać również token ... ?

impl Connection {


    pub fn new(stream: TcpStream) -> Connection {

        Connection {
            stream    : stream,
            keep_alive: false,                                //TODO - stąd powinien wylecieć ten parametr
            mode      : ConnectionMode::ReadingRequest([0u8; 2048], 0),
        }
    }

    fn make(stream: TcpStream, keep_alive: bool, mode: ConnectionMode) -> Connection {

        Connection {
            stream    : stream,
            keep_alive: keep_alive,                                //TODO - stąd powinien wylecieć ten parametr
            mode      : mode,
        }
    }

    fn replace_mode(self, mode: ConnectionMode) -> Connection {
        Connection {
            stream    : self.stream,
            keep_alive: self.keep_alive,
            mode      : mode,
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

            ConnectionMode::WaitingForServerResponse => {

                //TODO - występuje kopiowanie pamięci, znaleźć lepszy sposób na konwersję tych danych

                let mut resp_vec: Vec<u8> = Vec::new();

                for byte in response.as_bytes() {
                    resp_vec.push(byte.clone());
                }

                self.replace_mode(ConnectionMode::SendingResponse(resp_vec, 0))
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
            
            ConnectionMode::ReadingRequest(_, _)     => Event::Read,
            ConnectionMode::WaitingForServerResponse => Event::None,
            ConnectionMode::SendingResponse(_, _)    => Event::Write,
            ConnectionMode::Close                    => Event::None
        }
    }

    pub fn ready(self, events: EventSet, tok: Token) -> (Connection, Option<Request>) {
        
        if events.is_error() {
            println!("EVENT ERROR {}", tok.as_usize());
            panic!("TODO");
        }

        if events.is_hup() {
            println!("EVENT HUP - close - {} {:?}", tok.as_usize(), events);
            return (self.replace_mode(ConnectionMode::Close), None);
        }
        
        self.transform(events)
    }



    fn transform(self, events: EventSet) -> (Connection, Option<Request>) {

        match self.mode {

            ConnectionMode::ReadingRequest(buf, done) => {

                transform_from_waiting_for_user(self.stream, self.keep_alive, events, buf, done)
            }

            ConnectionMode::WaitingForServerResponse => {

                (Connection::make(self.stream, self.keep_alive, ConnectionMode::WaitingForServerResponse), None)
            }
            
            ConnectionMode::SendingResponse(str, done) => {

                (transform_from_sending_to_user(self.stream, self.keep_alive, events, str, done), None)
            },

            ConnectionMode::Close => {

                (Connection::make(self.stream, self.keep_alive, ConnectionMode::Close), None)
            }
        }
    }
}

fn transform_from_waiting_for_user(mut stream: TcpStream, keep_alive: bool, events: EventSet, mut buf: [u8; 2048], done: usize) -> (Connection, Option<Request>) {

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

                                    let keep_alive = request.is_header_set("Connection".to_string(), "keep-alive".to_string());

                                    (Connection::make(stream, keep_alive, ConnectionMode::WaitingForServerResponse), Some(request))
                                }

                                Err(mess) => {

//TODO - błąd 400
//trzeba też zamknąć połączenie

                                    (Connection::make(stream, keep_alive, ConnectionMode::ReadingRequest(buf, done)), None)
                                }
                            }
                        }

                                                            //częściowe parsowanie
                        Ok(httparse::Status::Partial) => {

                            (Connection::make(stream, keep_alive, ConnectionMode::ReadingRequest(buf, done)), None)
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

                            (Connection::make(stream, keep_alive, (ConnectionMode::ReadingRequest(buf, done))), None)
                        }
                    }


                    //uruchom parser
                        //jeśli się udało sparsować, to git

                    //jeśli osiągneliśmy całkowity rozmiar bufora a mimo to nie udało się sparsować danych
                        //to rzuć błędem że nieprawidłowe zapytanie

                } else {

                    (Connection::make(stream, keep_alive, (ConnectionMode::ReadingRequest(buf, done))), None)
                }
            }

            Ok(None) => {
                println!("no data");
                (Connection::make(stream, keep_alive, (ConnectionMode::ReadingRequest(buf, done))), None)
            }

            Err(err) => {
                println!("error read from socket {:?}", err);
                (Connection::make(stream, keep_alive, (ConnectionMode::ReadingRequest(buf, done))), None)
            }
        }



        //czytaj, odczytane dane przekaż do parsera
        //jeśli otrzymalismy poprawny obiekt requestu to :
            // przełącz stan tego obiektu połączenia, na oczekiwanie na dane z serwera
            // wyślij kanałem odpowiednią informację o requescie
            // zwróć informację na zewnątrz tej funkcji że nic się nie dzieje z tym połaczeniem

    } else {

        //trzeba też ustawić jakiś timeout czekania na dane od użytkownika

        return (Connection::make(stream, keep_alive, ConnectionMode::ReadingRequest(buf, done)), None);
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
                            return Connection::make(stream, keep_alive, (ConnectionMode::ReadingRequest([0u8; 2048], 0)));

                                                    //close connection
                        } else {
                            return Connection::make(stream, keep_alive, (ConnectionMode::Close));
                        }

                    } else if done < str.len() {

                        return Connection::make(stream, keep_alive, (ConnectionMode::SendingResponse(str, done)));

                    } else {

                        unreachable!();
                    }

                } else {

                    return Connection::make(stream, keep_alive, ConnectionMode::SendingResponse(str, done));
                }
            }

            Ok(None) => {

                println!("empty write");
                Connection::make(stream, keep_alive, ConnectionMode::SendingResponse(str, done))
            }

            Err(err) => {

                println!("error write to socket {:?}", err);
                Connection::make(stream, keep_alive, ConnectionMode::SendingResponse(str, done))
            }
        }

    } else {

        Connection::make(stream, keep_alive, ConnectionMode::SendingResponse(str, done))
    }
}
