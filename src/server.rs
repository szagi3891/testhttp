//mod server;

use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
//use mio::util::Slab;
use std::str;
use std::collections::HashMap;

use std::thread;
use std::time::Duration;

use connection::{Connection, ConnectionTransform};
use token_gen::TokenGen;

//use connect::


/*
    keep alive
    kompresja

    utworzenie soketu nas<b3>uchuj<b9>cego

    4 nowe event loopy
        nowy event lopp z po<bf>yczeniem tego soketu

    wysy<b3>aj<b9> kana<b3>em informacj<ea> o requestach do przetworzenia
        request :
            request        - request - do obs<b3>u<bf>enia
            time        - czas zapytania
            kana<b3> zwrotny - na kt<f3>ry zostanie przes<b3>ana odpowied<9f> do przes<b3>ania
*/

/*
    80
    443 - serwer z dekodowaniem certyfikatu -> a potem na http2

                            https://github.com/seanmonstar/httparse        - bezstanowy parser

    https://github.com/nbaksalyar/rust-streaming-http-parser    - nak<b3>adka na joyent parser
*/



//to co przeczytali<9c>my trafia do bufora
//parser przetwarzaa
//je<9c>li otrzymali<9c>my prawid<b3>ow<b9> warto<9c><e6> requestu, to zamknij czytanie i otw<f3>rz wysy<b3>anie
//obiekt requestu wy<9c>lij kana<b3>em na zewn<ea>trzny <9c>wiat

    //zewn<ea>trzny <9c>wiat, obiet requestu
        //ma token, ma kana<b3> kt<f3>rym mo<bf>emy si<ea> skomunikowa<e6> z powrotem
    //gdy wy<9c>lemy nowe dane odpowiedzi na ten obiekt, to obiekt musi zje<9c><e6> sam siebie (tylko raz mo<bf>na wys<b3>a<e6> odpowied<9f>)

//je<9c>li mamy keep alive, to utrzymujemy po<b3><b9>czenie i czekamy na nowe dane
//lub jesli klient si<ea> roz<b3><b9>czy<b3> to wyrzucamy obiekt po<b3><b9>czenia


//wykorzysta<e6> Slab<Connection> do trzymania puli po<b3><b9>cze<f1>


/*
https://github.com/carllerche/mio-examples/blob/master/ping_pong/src/main.rs
https://github.com/carllerche/mio/blob/master/test/test_close_on_drop.rs

https://github.com/carllerche/mio/blob/master/src/handler.rs

https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html
https://github.com/nbaksalyar/rust-chat/blob/part-1/src/main.rs


if hint.is_hup() {
    si<ea> roz<b3><b9>czy<b3>
*/


///&mut i32 to &'a mut i32, they<92>re the same





// Define a handler to process the events
pub struct MyHandler {
    token    : Token,
    server   : TcpListener,
    hash     : HashMap<Token, Connection>,
    tokens   : TokenGen
}


impl MyHandler {

    pub fn new(ip: &String) -> MyHandler {

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

        println!("serwer si<ea> zg<b3>osi<b3>");

        match self.server.accept() {

            Ok(Some((stream, addr))) => {

                let tok = self.tokens.get();
                let mut connection = Connection::new(stream);

                event_loop.register(&connection.stream, tok, EventSet::all(), PollOpt::edge());

                self.hash.insert(tok, connection);

                println!("nowe po<b3><b9>czenie : {}", addr);
            }

            Ok(None) => {

                println!("brak nowego po<b3><b9>czenia");
            }

            Err(e) => {

                println!("co<9c> posz<b3>o nie tak jak trzeba {}", e);
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
            je<9c>li tryb czekania na dane od u<bf>ytkownika, wejd<9f> w tryb -> czytaj i czekaj na roz<b3><b9>czenie
            je<9c>li request, przechod<9f> w -> tryb czekania tylko na zamkni<ea>cie
            je<9c>li dane do u<bf>ytkownika, przejd<9f> w -> tryb pisania lub czekaj na zamkniecie

            dodatkowo inne tryby uwzgl<ea>dni<e6>
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
