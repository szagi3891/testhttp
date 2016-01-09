mod statichttp;

use chan;
use miohttp;

use std::{io, thread};
use std::boxed::FnBox;
use std::path::Path;
use std::process;
use simple_signal::{Signals, Signal};

use miohttp::{response, log};


pub fn run_main() {
        
    let addres = "0.0.0.0:2222";
    
    println!("server running - {}", &addres);

    let wait_group = chan::WaitGroup::new();
    
    let exit_code = run(addres.to_owned(), &wait_group);

    // All channels dropped, wait for workers to end.
    log::debug(format!("Waiting for workers to end..."));
    wait_group.wait();
    log::info(format!("Bye."));
    
    process::exit(exit_code);
}


    // Scope for channels, after which they are dropped, so all threads begins to end.

fn run(addres: String, wait_group: &chan::WaitGroup) -> i32 {
    
    let (tx_request, rx_request) = chan::async();       //channel::<request::Request>();

    match miohttp::server::MyHandler::new(&addres, 4000, 4000, tx_request) {
        Ok(_) => { },
        Err(err) => {
            // Return real OS error to shell
            return err.raw_os_error().unwrap_or(-1)
        }
    }

    let (ctrl_c_tx1, ctrl_c_rx1) = chan::sync(0);
    let (ctrl_c_tx2, ctrl_c_rx2) = chan::sync(0);

    Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {

        log::debug(format!("Termination signal catched."));

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

    match thread::Builder::new().name("<StaticHttp master>".to_string()).spawn(move || {
        statichttp::run(wg, rx_files_path, tx_files_data);
    }) {
        Ok(join_handle) => join_handle,
        Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
    };


    loop {

        chan_select! {

            ctrl_c_rx1.recv() => {

                log::info(format!("Shutting down!"));
                ctrl_c_tx2.send(());
                return 0;
            },

            rx_request.recv() -> to_handle => {

                match to_handle {

                    Some(request) => {

                        let path_src = "./static".to_owned() + request.path.trim();

                        log::info(format!("Path requested: {}", &path_src));

                        tx_files_path.send((path_src.clone(), Box::new(move|data: Result<Vec<u8>, io::Error>|{

                            match data {

                                Ok(buffer) => {

                                    let buffer = buffer.to_owned();

                                    let path         = Path::new(&path_src);
                                    let content_type = response::Type::create_from_path(&path);

                                    log::info(format!("200, {}, {}", content_type, request.path));

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

                                            log::error(format!("errrrr {:?}", err));
                                        }
                                    }

                                }
                            }
                        })));
                    }

                    None => {

                        //TODO
                        //println!("wyparował nadawca requestów");
                        return 0;
                    }
                }
            },

            rx_files_data.recv() -> data => {

                log::debug(format!("Received file data"));

                match data {

                    Some((result, callback)) => {
                        callback.call_box((result,));
                    }

                    None => {

                        //TODO
                        log::info(format!("wyparował nadawca requestów"));
                        return 0;
                    }
                }
            }
        }
    };
}