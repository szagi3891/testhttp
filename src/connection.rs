use mio::{Token, EventLoop, EventSet, PollOpt, Handler, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
//use mio::util::Slab;
use std::str;
use std::collections::HashMap;


/*
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

}*/



enum ConnectionMode {

    //WaitingForDataUser(Parser<HttpParser>),         // oczekiwanie na dane od użytkownika

    WaitingForDataUser([u8; 2048], usize),
    
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
    pub stream : TcpStream,

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


impl Connection {

    pub fn new(stream: TcpStream) -> Connection {

        Connection {
            mode   : ConnectionMode::WaitingForDataUser([0u8; 2048], 0),
            stream : stream,
        }
    }
    
    pub fn ready(&mut self, events: EventSet) -> ConnectionTransform {
		
        match *(&self.mode) {
			
            ConnectionMode::WaitingForDataUser(ref mut buf, ref mut done) => {
				
				if events.is_readable() {
					
					println!("trzeba spróbować przeczytać coś z socketu");
					
                    
					let total = buf.len();
                    
					match self.stream.try_read(&mut buf[(*done)..total]) {
						
						Ok(Some(size)) => {
                            
                            *done = *done + size;
                            
							println!("odczytano : {}", size);
                            
                            
                    
                            //uruchom parser
                                //jeśli się udało sparsować, to git

                            //jeśli osiągneliśmy całkowity rozmiar bufora a mimo to nie udało się sparsować danych
                                //to rzuć błędem że nieprawidłowe zapytanie
						}
						
						Ok(None) => {
							println!("brak danych");
						}
                        
						Err(err) => {
							println!("błąd czytania ze strumienia {:?}", err);
						}
					}
					
                    
                    
					//czytaj, odczytane dane przekaż do parsera
					//jeśli otrzymalismy poprawny obiekt requestu to :
						// przełącz stan tego obiektu połączenia, na oczekiwanie na dane z serwera
						// wyślij kanałem odpowiednią informację o requescie
						// zwróć informację na zewnątrz tej funkcji że nic się nie dzieje z tym połaczeniem
					
					
					//parse(&mut self, data: &[u8]) -> usize
					//jeśli usize jest > 0 to znaczy że się udało parsowanie
					
					/*
So really, 'allocation-free' means, make any allocations you want beforehand, and then give me a slice. (Hyper creates a stack array of [Header; 100], for instance).

					https://github.com/seanmonstar/httparse
					
									httpparse w hyper
					https://github.com/hyperium/hyper/blob/master/src/http/h1.rs
					*/
					
					
					/*
					fn read(stream : &mut TcpStream, total : usize) -> Option<Vec<u8>>
					{
						let mut buffer = Vec::with_capacity(total);
						let mut done   = 0;

						unsafe { 
							buffer.set_len(total)
						}

						while done < total {

							if let Ok(count) = stream.read(&mut buffer[done..total]) {
								done += count;    
							} else {
								break;
							}   
						}

						if done == total {
							Some(buffer)
						} else {
							None
						}   
					}
					*/
				}
				
				//trzeba też ustawić jakiś timeout czekania na dane od użytkownika
				
                ConnectionTransform::None
            }
			
            ConnectionMode::WaitingForDataServer(keep_alive) => {
                ConnectionTransform::None
            }
			
            ConnectionMode::DataToSendUser(keep_alive, ref str)  => {
				
				if events.is_writable() {

					println!("zapisuję strumień");

					//println!("strumień : {:?}", &self.token);
					//println!("strumień zapisuję : {:?}", &self.token);

					let response = format!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\n\r\nCześć czołem");

					self.stream.try_write(response.as_bytes()).unwrap();

					//jeśli udany zapis, to zmień stan na oczekiwanie danych od użytkownika lub zamknij to połączenie
				}
				
				ConnectionTransform::None
            }
        }
    }
}

