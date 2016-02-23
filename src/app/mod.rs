mod api;
mod worker;

use std::process;
use simple_signal::{Signals, Signal};
use channels_async::{Chan, Sender, Receiver, Select};

use asynchttp::{miohttp,log};
use asynchttp::async::{spawn};
use asynchttp::async::Manager;
use asynchttp::miohttp::request::Request;

use app::api::Request  as apiRequest;
use app::api::Response as apiResponse;


pub fn run_main() {
        
    let addres = "0.0.0.0:2222";
    
    println!("server running - {}", &addres);
    
    let exit_code = run(addres.to_owned());

    // All channels dropped, wait for workers to end.
    //log::debug(format!("Waiting for workers to end..."));

    log::info(format!("Bye."));
    
    process::exit(exit_code);
}



fn run(addres: String) -> i32 {
    
    let (request_producer, request_consumer) = Chan::new().couple();

    let thread_name = "<EventLoop>".to_owned();

    match spawn(thread_name, move ||{
        
        miohttp::server::MyHandler::new(&addres, 4000, 4000, request_producer);
        
        //tutaj trzeba odebrać błąd, a następnie go odpowiednio sformatować i wyrzucić w loga
        
    }) {
        Ok(join_handle) => join_handle,
        Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
    };
    
    
    /*
        TODO - zarządca
        
        jeśli pod rząd nie może uruchomić programu 3 razy, to fail
        w przypadku gdy poleci panic, to wznawiaj taki proces ...
    */
    
    
    
    // Return real OS error to shell, return err.raw_os_error().unwrap_or(-1)
    
    let (api_request_producer , api_request_consumer)  = Chan::new().couple();
    let (api_response_producer, api_response_consumer) = Chan::new().couple();
    
    {
        let api_request_consumer  = api_request_consumer.clone();
        let api_response_producer = api_response_producer.clone();
        
        let manager_api = Manager::new("api".to_owned(), 1, Box::new(move|thread_name: String|{

            let api_request_consumer  = api_request_consumer.clone();
            let api_response_producer = api_response_producer.clone();

            match spawn(thread_name.to_owned(), move ||{
                api::run(api_request_consumer, api_response_producer);
            }) {
                Ok(join_handle) => join_handle,
                Err(err) => panic!("Can't spawn {}: {}", thread_name, err),
            };
        }));
    }
    
    
    
    //TODO - nazwę wątku wzbogacić o licznik
    
    let manager_workers = Manager::new("worker".to_owned(), 4, Box::new(move|thread_name: String|{
        
        let request_consumer      = request_consumer.clone();
        let api_request_producer  = api_request_producer.clone();
        let api_response_consumer = api_response_consumer.clone();
        
        match spawn(thread_name, move ||{
            run_worker(request_consumer, api_request_producer, api_response_consumer);
        }) {
            Ok(join_handle) => join_handle,
            Err(err) => panic!("Can't spawn api spawner: {}", err),
        };
    }));
    
    
    let (sigterm_sender,  sigterm_receiver ) = Chan::new().couple();
    let (shutdown_sender, shutdown_receiver) = Chan::new().couple();
    
    Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {

        log::debug(format!("Termination signal catched."));

        sigterm_sender.send(());
        
        // oczekuj na zakończenie procedury wyłączania
        let _ = shutdown_receiver.get();
    });
    
    
    // główna pętla sterująca podwątkami
    loop {
        
        let _ = sigterm_receiver.get();

        log::info(format!("Shutting down!"));
        
        //TODO - czekaj aż wsystkie taski się zakończą ...
        
        //TODO - manager_api --> off
        //TODO - manager_workers -> off
        
        let _ = shutdown_sender.send(());
        return 0;
    }
}


fn run_worker(request_consumer: Receiver<Request>, api_request_producer: Sender<apiRequest>, api_response_consumer: Receiver<apiResponse>) {
    
    enum Out {
        Result1(Request),
        Result2(apiResponse),
    }
    
    let select: Select<Out> = Select::new();
    
    select.add(request_consumer     , Box::new(|value:     Request| Out::Result1(value)));
    select.add(api_response_consumer, Box::new(|value: apiResponse| Out::Result2(value)));
    
    loop {
        match select.get() {
            Out::Result1(request) => {
                worker::render_request(request, &api_request_producer);
            },
            Out::Result2(api::Response::GetFile(result, callback)) => {
                log::debug(format!("Received file data"));
                callback.call_box((result,));
            }
        }
    }
}


