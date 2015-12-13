use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
//use mio::util::Slab;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

extern crate mio;
extern crate httparse;

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

struct TokenGen {
    count : usize
}

impl TokenGen {
    fn new() -> TokenGen {
        TokenGen{count : 0}
    }
    
    fn get(&mut self) -> Token {
        
        let curr = self.count;
        self.count = self.count + 1;
        
        Token(curr)
    }
}

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


enum ConnectionMode {
    ForUserData,            //oczekiwanie na dane od użytkownika
    ForServerData,          //oczekiwanie na dane z odpowiedzią od serwera
}

/*
    przychodza dane z nowego połączenia
    tworzymy bufor w który są wkładane te dane, następnie przekazujemy bufor dalej do tablicy połączenia
*/
struct Connection<'a> {
    
    mode    : ConnectionMode,
    headers : &'a [httparse::Header<'a>],
    parser  : httparse::Request<'a, 'a>,
    //buf     : str,
    
    /*
    parse - nowe dane
        na wyjściu otrzmujemy opcję z obiektem requestu
    writeResponse   - zapisywanie w strumień odpowiedzi
        zjadanie obiektu który był przekazany dalej
    
    
    http://seanmonstar.com/
        info o bezstanowości httparse
        
    https://github.com/hyperium/hyper/blob/master/src/buffer.rs
        sprawdzić jak hyper sobie radzi z parsowaniem danych ...
    */
}




/*
impl Connection {
    
    fn new() -> Connection {
        let mut headers = [httparse::EMPTY_HEADER; 256];
        let mut req = httparse::Request::new(&mut headers);
    }  
}
*/


// Define a handler to process the events
struct MyHandler {
    token    : Token,
    server   : TcpListener,
    hash     : HashMap<Token, TcpStream>,
    //hash     : Slab<TcpStream>,
    tokens   : TokenGen
}


impl MyHandler {

    //fn new(ip: &'static str) -> MyHandler {
    //fn new(ip: &str) -> MyHandler {
    fn new(ip: &String) -> MyHandler {

        let mut tokens = TokenGen::new();

        let mut event_loop = EventLoop::new().unwrap();


        let addr = ip.parse().unwrap();

        let server = TcpListener::bind(&addr).unwrap();

        let token = tokens.get();

        event_loop.register(&server, token, EventSet::readable(), PollOpt::edge()).unwrap();

        let mut inst = MyHandler{token: token, server: server, hash: HashMap::new(), tokens:tokens};
        //let mut inst = MyHandler{token: token, server: server, hash: Slab::new(1024 * 10), tokens:tokens};
        
        // Start handling events
        event_loop.run(&mut inst).unwrap();

        inst
    }

}

impl Handler for MyHandler {

    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, events: EventSet) {
        
        if token == self.token {

            println!("serwer się zgłosił");
            
            match self.server.accept() {

                Ok(Some((stream, addr))) => {

                    let tok = self.tokens.get();
                    
                    event_loop.register(&stream, tok, EventSet::all(), PollOpt::edge());

                    self.hash.insert(tok, stream);

                    println!("nowe połączenie : {}", addr);
                }

                Ok(None) => {

                    println!("brak nowego połączenia");
                }

                Err(e) => {

                    println!("coś poszło nie tak jak trzeba {}", e);
                }
            }

        } else {
            
            if events.is_writable() {
                
                match self.hash.remove(&token) {

                    Some(mut stream) => {

                        println!("strumień : {:?}", &token);

                        thread::spawn(move || {
                            // some work here

                                                            //5 sekund
                            thread::sleep(Duration::new(5, 0));

                            println!("strumień zapisuję : {:?}", &token);

                            let response = std::fmt::format(format_args!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\n\r\nCześć czołem"));

                            stream.try_write(response.as_bytes()).unwrap();	
                        });


                    }
                    None => {
                        println!("Brak strumienia pod tym hashem: {:?}", &token);
                    }
                }
                return;
            }

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
        }

        //println!("w ready");
    }
}



fn main() {
    	
    println!("Hello, world! - 3");
	
    MyHandler::new(&"127.0.0.1:13265".to_string());
	
	println!("po starcie");
}

