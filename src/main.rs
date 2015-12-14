use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
//use mio::util::Slab;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

extern crate mio;
extern crate http_muncher;

use http_muncher::{Parser, ParserHandler};

mod token_gen;
use token_gen::TokenGen;

/*
	keep alive
	kompresja
	
	utworzenie soketu nasłuchującego

	4 nowe event loopy
		nowy event lopp z pożyczeniem tego soketu

	wysyłają kanałem informację o requestach do przetworzenia
		request :
			request		- request - do obsłużenia
			time		- czas zapytania
			kanał zwrotny - na który zostanie przesłana odpowiedź do przesłania
*/

/*
    80
    443 - serwer z dekodowaniem certyfikatu -> a potem na http2
    
                            https://github.com/seanmonstar/httparse		- bezstanowy parser
    
	https://github.com/nbaksalyar/rust-streaming-http-parser	- nakładka na joyent parser
*/



//to co przeczytaliśmy trafia do bufora
//parser przetwarzaa
//jeśli otrzymaliśmy prawidłową wartość requestu, to zamknij czytanie i otwórz wysyłanie
//obiekt requestu wyślij kanałem na zewnętrzny świat

    //zewnętrzny świat, obiet requestu
        //ma token, ma kanał którym możemy się skomunikować z powrotem
    //gdy wyślemy nowe dane odpowiedzi na ten obiekt, to obiekt musi zjeść sam siebie (tylko raz można wysłać odpowiedź)

//jeśli mamy keep alive, to utrzymujemy połączenie i czekamy na nowe dane
//lub jesli klient się rozłączył to wyrzucamy obiekt połączenia


//wykorzystać Slab<Connection> do trzymania puli połączeń


/*
https://github.com/carllerche/mio-examples/blob/master/ping_pong/src/main.rs
https://github.com/carllerche/mio/blob/master/test/test_close_on_drop.rs

https://github.com/carllerche/mio/blob/master/src/handler.rs

https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html
https://github.com/nbaksalyar/rust-chat/blob/part-1/src/main.rs


if hint.is_hup() {
    się rozłączył
*/


///&mut i32 to &'a mut i32, they’re the same




struct HttpParser {
    current_key: Option<String>,
    headers: HashMap<String, String>,
}

impl ParserHandler for HttpParser {
    
    fn on_header_field(&mut self, s: &[u8]) -> bool {
        self.current_key = Some(std::str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_header_value(&mut self, s: &[u8]) -> bool {
        self.headers.insert(self.current_key.clone().unwrap(),
                    std::str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_headers_complete(&mut self) -> bool {
        false
    }
}



enum ConnectionMode {
    
    WaitinhForDataUser(Parser<HttpParser>),         //oczekiwanie na dane od użytkownika
    
    WaitinhForDataServer(bool),                     //oczekiwanie na wygenerowanie danych z odpowiedzią od serwera
                                                    //bool - oznacza czy był ustawiony keep alivee
    
    DataToSendUser(bool, String),                   //siedzą dane gotowe do wysłania dla użytkownika
                                                    //bool - oznacza czy był ustawiony keep alivee
}


/*
struct request {
    //parser
    //metody dostępowe
}*/


enum ConnectionTransform {
    None,
    Close,
    Write,
    Read,
}


struct Connection {
    mode       : ConnectionMode,
    stream     : TcpStream,
    
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
    
    fn new(stream: TcpStream) -> Connection {
        
        Connection {
            mode   : ConnectionMode::WaitinhForDataUser(Connection::new_parser()),
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
    
    fn ready(& mut self, events: EventSet) -> ConnectionTransform {
        
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
            
            ConnectionMode::WaitinhForDataServer(keep_alive) => {
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
            
            ConnectionMode::WaitinhForDataUser(ref mut parser) => {
                
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



// Define a handler to process the events
struct MyHandler {
    token    : Token,
    server   : TcpListener,
    hash     : HashMap<Token, Connection>,
    tokens   : TokenGen
}


impl MyHandler {
    
    fn new(ip: &String) -> MyHandler {

        let mut tokens = TokenGen::new();

        let mut event_loop = EventLoop::new().unwrap();

        let addr = ip.parse().unwrap();

        let server = TcpListener::bind(&addr).unwrap();

        let token = tokens.get();

        event_loop.register(&server, token, EventSet::readable(), PollOpt::edge()).unwrap();

        let mut inst = MyHandler{token: token, server: server, hash: HashMap::new(), tokens:tokens};
        //let mut inst = MyHandler{token: token, server: server, hash: Slab::new(1024 * 10), tokens:tokens};
        
        
        event_loop.run(&mut inst).unwrap();

        inst
    }


    fn new_connection(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
        
        println!("serwer się zgłosił");

        match self.server.accept() {

            Ok(Some((stream, addr))) => {

                let tok = self.tokens.get();
                let mut connection = Connection::new(stream);

                event_loop.register(&connection.stream, tok, EventSet::all(), PollOpt::edge());

                self.hash.insert(tok, connection);

                println!("nowe połączenie : {}", addr);
            }

            Ok(None) => {

                println!("brak nowego połączenia");
            }

            Err(e) => {

                println!("coś poszło nie tak jak trzeba {}", e);
            }
        }
    }
    
    fn socket_ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
        
        //get

        let closeConn = match self.hash.get_mut(&token) {

            Some(mut connection) => {

                connection.ready(events)
            }
            None => {
                println!("Brak strumienia pod tym hashem: {:?}", &token);
                ConnectionTransform::None
            }
        };

        /*
            jeśli tryb czekania na dane od użytkownika, wejdź w tryb -> czytaj i czekaj na rozłączenie
            jeśli request, przechodź w -> tryb czekania tylko na zamknięcie
            jeśli dane do użytkownika, przejdź w -> tryb pisania lub czekaj na zamkniecie

            dodatkowo inne tryby uwzględnić
        */

        match closeConn {
            ConnectionTransform::None => {
            }

            ConnectionTransform::Write => {
                //przestawienie w tryb czytania z socketu
            }

            ConnectionTransform::Read => {

                //przestawienie w tryb pisania do soketu
            }

            ConnectionTransform::Close => {
                let _ = self.hash.remove(&token);
            }
        }
    }
}


impl Handler for MyHandler {

    type Timeout = ();
    type Message = ();
    
    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
        
        if token == self.token {
            
            self.new_connection(event_loop, token, events);

        } else {
            self.socket_ready(event_loop, token, events);
        }
    }
    
    
    
}



fn main() {
    	
    println!("Hello, world! - 127.0.0.1:13265");
	
    MyHandler::new(&"127.0.0.1:13265".to_string());
	
	println!("po starcie");
}


/*                        thread::spawn(move || {
                            // some work here

                                                            //5 sekund
                            thread::sleep(Duration::new(5, 0));
*/
//                        });

            /*
            match self.hash.get_mut(&token) {

                Some(stream) => {

                    if events.is_readable() {

                        println!("czytam");


                        let mut buf = [0u8; 2048];

                        //let mut buf = ByteBuf::mut_with_capacity(2048);
                        //let mut buf: String = String::new();

                        //match Strem.recv_from(buf) {
                        match stream.try_read(&mut buf) {
                        //match Strem.read(&mut buf) {

                            Ok(Some(size)) => {

                                println!("odczytano : {}", size);

                            }
                            Ok(None) => {
                                println!("brak danych");
                            }

                            Err(err) => {
                                println!("błąd czytania ze strumienia {:?}", err);
                            }
                        }
                    }

                    if events.is_writable() {

                        println!("piszę");

                        //fn write(&mut self, buf: &[u8]) -> Result<usize>

                        let response = std::fmt::format(format_args!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: application/xhtml+xml; charset=utf-8\r\n\r\nCześć czołem"));

                        stream.try_write(response.as_bytes()).unwrap();	
                    }
                }
                None => {
                    println!("brak strumienia");
                }
            }
            */