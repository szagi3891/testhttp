use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
//use mio::util::Slab;
use std::str;
use std::collections::HashMap;
use http_muncher::{Parser, ParserHandler};


struct HttpParser {
    current_key: Option<String>,
    headers: HashMap<String, String>,
}

impl ParserHandler for HttpParser {

    fn on_header_field(&mut self, s: &[u8]) -> bool {
        self.current_key = Some(str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_header_value(&mut self, s: &[u8]) -> bool {
        self.headers.insert(
            self.current_key.clone().unwrap(),
            str::from_utf8(s).unwrap().to_string()
        );
        true
    }

    fn on_headers_complete(&mut self) -> bool {
        false
    }

}



enum ConnectionMode {

    WaitingForDataUser(Parser<HttpParser>),         // oczekiwanie na dane od użytkownika

    WaitingForDataServer(bool),                     // oczekiwanie na wygenerowanie danych z odpowiedzią od serwera
                                                    // bool - oznacza czy był ustawiony keep alivee

    DataToSendUser(bool, String),                   // siedzą dane gotowe do wysłania dla użytkownika
                                                    // bool - oznacza czy był ustawiony keep alivee
}


/*
struct request {
    //parser
    //metody dostępowe
}*/


pub enum ConnectionTransform {
    None,
    Close,
    Write,
    Read,
}


pub struct Connection {
    mode       : ConnectionMode,
    pub stream     : TcpStream,

    /*
    parse - nowe dane
        na wyjściu otrzmujemy opcję z obiektem requestu
    writeResponse   - zapisywanie w strumień odpowiedzi
        zjadanie obiektu który był przekazany dalej


    http://seanmonstar.com/
        info o bezstanowości httparse

    https://github.com/hyperium/hyper/blob/master/src/buffer.rs
        sprawdzić jak hyper sobie radzi z parsowaniem danych ...

    https://github.com/nbaksalyar/rust-chat/blob/part-1/src/main.rs#L2
        dobrze zaimplementowane mio
    */
}


/*
struct ConnectionResult {
}
    nic
    zamknij socket
    obiekt requestu
*/


impl Connection {

    pub fn new(stream: TcpStream) -> Connection {

        Connection {
            mode   : ConnectionMode::WaitingForDataUser(Connection::new_parser()),
            stream : stream,
        }
    }

    fn new_parser() -> Parser<HttpParser> {

        let http_parser_inst = HttpParser {
            current_key: None,
            headers: HashMap::new(),
        };

        Parser::request(http_parser_inst)
    }

    pub fn ready(& mut self, events: EventSet) -> ConnectionTransform {

        if events.is_writable() {

            self.run_writable()

        } else if events.is_readable() {

            self.run_readable()

        } else {

            panic!("{}", "nieznane wydarzenie");
        }
    }


    fn run_writable(& mut self) -> ConnectionTransform {

        match *(&self.mode) {

            ConnectionMode::WaitingForDataServer(keep_alive) => {
                ConnectionTransform::None
            }

            ConnectionMode::DataToSendUser(keep_alive, ref str)  => {

                println!("zapisuję strumień");

                //println!("strumień : {:?}", &self.token);
                //println!("strumień zapisuję : {:?}", &self.token);

                let response = format!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\n\r\nCześć czołem");

                self.stream.try_write(response.as_bytes()).unwrap();

                //jeśli udany zapis, to zmień stan na oczekiwanie danych od użytkownika lub zamknij to połączenie

                ConnectionTransform::None
            }

            _ => {
                //ignoruję inne stany
                ConnectionTransform::None
            }
        }
    }

    fn run_readable(& mut self) -> ConnectionTransform {

        match *(&mut self.mode) {

            ConnectionMode::WaitingForDataUser(ref mut parser) => {

                ConnectionTransform::None
            }

            _ => {
                ConnectionTransform::None
            }
        }

        /*
        if self.mode == ConnectionMode::ForUserData {

            //parsuj
            //gdy się sparsujesz, to przełącz się z trybem
        }
        */
    }

    //fn parse() {
    //}
}

