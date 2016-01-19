mod api;
mod worker;

use std::process;
use simple_signal::{Signals, Signal};
use comm;
use comm::select::{Select, Selectable};

use asynchttp::{miohttp,log};
use asynchttp::async::{spawn};
use asynchttp::miohttp::{request, channels};


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
    
    let request_channel = channels::RequestChannel::new(100);       // performance problems with nthreads > cores
    //let (request_producer, request_consumer) = comm::spmc::bounded_fast::new(100);  // unsafe
    //let (request_producer, request_consumer) = comm::spmc::unbounded::new();          // buffer overflow

    let thread_name = "<EventLoop>".to_owned();

    let request_producer = request_channel.clone();
    match spawn(thread_name, move ||{
        
        miohttp::server::MyHandler::new(&addres, 4000, 4000, request_producer);
        
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
    
    let api_request  = comm::mpmc::bounded::Channel::new(100);        //<(String, api::CallbackFD)>
    let api_response = comm::mpmc::bounded::Channel::new(100);        //<(api::FilesData, api::CallbackFD)>
    
    {
        let thread_name = "<api>".to_owned();
       
        let rx_api_request = api_request.clone();
        let tx_api_response = api_response.clone();

        match spawn(thread_name, move ||{
            api::run(rx_api_request, tx_api_response);
        }) {
            Ok(join_handle) => join_handle,
            Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
        };
    }
    
    
    /*
    let manager_workers =  Manager::new("worker".to_owned(), 4, Box::new(move|thread_name: String|{
            
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
    }));
    */
    
                                //np. 4 workery
    for _ in 0..20 {
        
        let thread_name = "<worker>".to_owned();
       
        let request_consumer = request_channel.clone();
        let tx_api_request   = api_request.clone();
        let rx_api_response  = api_response.clone();
        
        match spawn(thread_name, move ||{
            run_worker(request_consumer, tx_api_request, rx_api_response);
        }) {
            Ok(join_handle) => join_handle,
            Err(err) => panic!("Can't spawn api spawner: {}", err),
        };
    }
    
    
    
    let (sigterm_sender,  sigterm_receiver ) = comm::spsc::one_space::new();
    let (shutdown_sender, shutdown_receiver) = comm::spsc::one_space::new();

    {
        Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {

            log::debug(format!("Termination signal catched."));

            sigterm_sender.send(());

            // oczekuj na zakończenie procedury wyłączania
            let _ = shutdown_receiver.recv_sync();
        });
    }
    
    // główna pętla sterująca podwątkami
    loop {
        
        let _ = sigterm_receiver.recv_sync();

        log::info(format!("Shutting down!"));
        shutdown_sender.send(());
        return 0;
    }
}



fn run_worker<'a>(rx_request: comm::mpmc::bounded::Channel<'a, request::Request>, tx_api_request: comm::mpmc::bounded::Channel<'a, api::Request>, rx_api_response: comm::mpmc::bounded::Channel<'a, api::Response>) {
    
    let select = Select::new();
    select.add(&rx_request);
    select.add(&rx_api_response);

    loop {
        for &mut id in select.wait(&mut [0, 0]) {
            if id == rx_request.id() {

                match rx_request.recv_sync() {

                    Ok(request) => {
                        
                        worker::render_request(request, &tx_api_request);
                    }

                    Err(err) => {

                        //TODO
                        println!("ex_request channel error: {:?}", err);
                        return;
                    }
                }
            }

            else if id == rx_api_response.id() {
                
                match rx_api_response.recv_sync() {
                    
                    Ok(api::Response::GetFile(result, callback)) => {
                        
                        log::debug(format!("Received file data"));
                        callback.call_box((result,));
                    }
                    
                    Err(err) => {

                        //TODO
                        log::info(format!("rx_api_response channel error: {:?}", err));
                        return;
                    }
                }
            }
        }
    };
}


