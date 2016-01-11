mod api;
mod worker;

use chan::{self};
use asynchttp::{miohttp,log};
use asynchttp::miohttp::request;
use std::{process};
use simple_signal::{Signals, Signal};
use asynchttp::async::{spawn, Manager};

pub fn run_main() {
        
    let addres = "0.0.0.0:2222";
    
    println!("server running - {}", &addres);
    
    let exit_code = run(addres.to_owned());

    // All channels dropped, wait for workers to end.
    log::debug(format!("Waiting for workers to end..."));
    log::info(format!("Bye."));
    
    process::exit(exit_code);
}



fn run(addres: String) -> i32 {
    
    
    let (tx_request, rx_request) = chan::async();
    
    
    let thread_name = "<EventLoop>".to_owned();
        
    match spawn(thread_name, move ||{
        
        let tx_request = tx_request.clone();
        
        miohttp::server::MyHandler::new(&addres, 4000, 4000, tx_request);
        
        //tutaj trzeba odebrać błąd, a następnie go odpowiednio sformatować i wyrzucić w loga
        
    }) {
        Ok(join_handle) => join_handle,
        Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
    };
    
    
    
    /*
    
        zarządca
        jeśli pod rząd nie może uruchomić programu 3 razy, to fail
        
        manager::create(3)      -- twórz 3 procesy potomkowe

        manager.shoutdown();

        w przypadku gdy poleci panic, to wznawiaj taki proces ...
    */
    
    
    
    // Return real OS error to shell, return err.raw_os_error().unwrap_or(-1)
    
    let (tx_api_request , rx_api_request) = chan::async();         //<(String, api::CallbackFD)>
    let (tx_api_response, rx_api_response) = chan::async();         //<(api::FilesData, api::CallbackFD)>
    
    
    let thread_name = "<api>".to_owned();
    
    match spawn(thread_name, move ||{
        api::run(rx_api_request, tx_api_response);
    }) {
        Ok(join_handle) => join_handle,
        Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
    };
    
    
    
    /*
    let manager_workers = {
        
        let rx_request      = rx_request.clone();
        let tx_api_request  = tx_api_request.clone();
        let rx_api_response = rx_api_response.clone();
        
        Manager::new("worker".to_owned(), 4, Box::new(move|thread_name: String|{
            
            //let thread_name = "<worker>".to_owned();
            
            let rx_request      = rx_request.clone();
            let tx_api_request  = tx_api_request.clone();
            let rx_api_response = rx_api_response.clone();
            
            match spawn(thread_name, move ||{
                run_worker(rx_request, tx_api_request, rx_api_response);
            }) {
                Ok(join_handle) => join_handle,
                Err(err) => panic!("Can't spawn api spawner: {}", err),
            };
        }))
    };
    */

                                //np. 4 workery
    
    for _ in 0..4 {
        
        let thread_name = "<worker>".to_owned();
        
        let rx_request      = rx_request.clone();
        let tx_api_request  = tx_api_request.clone();
        let rx_api_response = rx_api_response.clone();

        match spawn(thread_name, move ||{
            run_worker(rx_request, tx_api_request, rx_api_response);
        }) {
            Ok(join_handle) => join_handle,
            Err(err) => panic!("Can't spawn api spawner: {}", err),
        };
    }
    
    
    
    let (ctrl_c_tx1, ctrl_c_rx1) = chan::sync(0);
    let (ctrl_c_tx2, ctrl_c_rx2) = chan::sync(0);

    Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {

        log::debug(format!("Termination signal catched."));

        ctrl_c_tx1.send(());
                                        //oczekuj na zakończenie procedury wyłączania
        let _ = ctrl_c_rx2.recv();
    });


    
                //główna pętla sterująca podwątkami
    loop {
        
        let _ = ctrl_c_rx1.recv();

        log::info(format!("Shutting down!"));
        ctrl_c_tx2.send(());
        return 0;
    }
}



fn run_worker(rx_request: chan::Receiver<request::Request>, tx_api_request: chan::Sender<api::Request>, rx_api_response: chan::Receiver<api::Response>) {
    
    loop {

        chan_select! {

            rx_request.recv() -> to_handle => {

                match to_handle {

                    Some(request) => {
                        
                        worker::render_request(request, &tx_api_request);
                    }

                    None => {

                        //TODO
                        //println!("wyparował nadawca requestów");
                        return;
                    }
                }
            },

            rx_api_response.recv() -> data => {
                
                match data {
                    
                    Some(api::Response::GetFile(result, callback)) => {
                        
                        log::debug(format!("Received file data"));
                        callback.call_box((result,));
                    }
                    
                    None => {

                        //TODO
                        log::info(format!("wyparował nadawca requestów"));
                        return;
                    }
                }
            }
        }
    };
}


