mod api;
mod worker;

use chan;
use asynchttp::{miohttp,log};
use std::{process, thread};
use simple_signal::{Signals, Signal};


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


    let (tx_files_path, rx_files_path) = chan::async();
    let (tx_files_data, rx_files_data) = chan::async();
    
    /*
    use app::api;
    
    let (tx_files_path, rx_files_path) = channel::<(String, api::CallbackFD)>();
    let (tx_files_data, rx_files_data) = channel::<(api::FilesData, api::CallbackFD)>();
    */

    let wg = wait_group.clone();

    match thread::Builder::new().name("<StaticHttp master>".to_string()).spawn(move || {
        api::run(wg, rx_files_path, tx_files_data);
    }) {
        Ok(join_handle) => join_handle,
        Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
    };

    
    //chan::Sender<request::Request>,
    
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
                        
                        worker::render_request(request, &tx_files_path);
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