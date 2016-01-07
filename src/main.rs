#![feature(fnbox)]

extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;
#[macro_use]
extern crate chan;

use std::{io, thread};
use std::boxed::FnBox;
use std::path::Path;
use simple_signal::{Signals, Signal};
use miohttp::response;

mod async;
mod miohttp;
mod statichttp;


fn main() {
    
    let addres = "0.0.0.0:2222";
    
    println!("server running - {}", &addres);

    let wait_group = chan::WaitGroup::new();

    // Scope for channels, after which they are dropped, so all threads begins to end.
    {
        let (tx_request, rx_request) = chan::async();       //channel::<request::Request>();

        miohttp::server::MyHandler::new(&addres.to_string(), 4000, 4000, tx_request);
        

        let (ctrl_c_tx1, ctrl_c_rx1) = chan::sync(0);
        let (ctrl_c_tx2, ctrl_c_rx2) = chan::sync(0);

        Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {

            println!("catch ctrl+c");
            
            ctrl_c_tx1.send(());
                                            //oczekuj na zakończenie procedury wyłączania
            let _ = ctrl_c_rx2.recv();
        });
        
        /*
            workery
                api

            worker od plików

            worker od bazy danych
        */
        
        /*

            model uproszczony
            
            worker plików
                pytanie o plik - kanałem
                odpowiedź o plik - kanałem, odpowiada clouserem do uruchomienia oraz danymi tego pliku
                dane pliku współdzielone za pomocą ARC (tylko do odczytu)

            proces workera ogólnego (w pełni asynchronicznego)
                tworzy nowy obiekt api (z namiarami na kanały workera plików)
                odbiera request - uruchamia główną metodę api
                odbiera clousera - uruchamia go
        */
        
        let (tx_files_path, rx_files_path) = chan::async();
        let (tx_files_data, rx_files_data) = chan::async();

        /*
        let (tx_files_path, rx_files_path) = channel::<(String, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>();
        let (tx_files_data, rx_files_data) = channel::<(Result<Vec<u8>, io::Error>, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>();
        */

        let wg = wait_group.clone();

        match thread::Builder::new().name("StaticHttp master".to_string()).spawn(move || {
            statichttp::run(wg, rx_files_path, tx_files_data);
        }) {
            Ok(join_handle) => join_handle,
            Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
        };


        loop {

            chan_select! {

                ctrl_c_rx1.recv() => {

                    println!("shutdown");
                    ctrl_c_tx2.send(());
                    break;
                },

                rx_request.recv() -> to_handle => {

                    match to_handle {
                        
                        Some(request) => {
                            
                            let path_src = "./static".to_owned() + request.path.trim();
                            
                            println!("ścieżka do zaserwowania {}", &path_src);
                            
                            tx_files_path.send((path_src.clone(), Box::new(move|data: Result<Vec<u8>, io::Error>|{
                                
                                match data {
                                    
                                    Ok(buffer) => {
                                        
                                        let buffer = buffer.to_owned();

                                        let path         = Path::new(&path_src);
                                        let content_type = response::Type::create_from_path(&path);
                                        
                                        println!("200, {}, {}", content_type, request.path);
                                        
                                        let response = response::Response::create_from_buf(response::Code::Code200, content_type, buffer);
                                        
                                        request.send(response);
                                    }
                                    
                                    Err(err) => {

                                        match err.kind() {

                                            io::ErrorKind::NotFound => {

                                                let mess     = "Not fund".to_string();
                                                let response = response::Response::create(response::Code::Code404, response::Type::TextHtml, mess);
                                                request.send(response);
                                            }
                                            _ => {

                                                println!("errrrr {:?}", err);
                                            }
                                        }

                                    }
                                }
                            })));
                        }

                        None => {

                            //TODO
                            //println!("wyparował nadawca requestów");
                            break;
                        }
                    }
                },

                rx_files_data.recv() -> data => {

                    println!("odebrało dane z plikiem");
                    
                    match data {
                        
                        Some((result, callback)) => {
                            callback.call_box((result,));
                        }

                        None => {

                            //TODO
                            println!("wyparował nadawca requestów");
                            break;
                        }
                    }
                }
            }
        }

    }

    // All channels dropped, wait for workers to end.
    println!("Waiting for workers to end...");
    wait_group.wait();
    println!("Bye.");
}

