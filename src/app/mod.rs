mod api;
mod worker;

use std::process;
use channels_async::{channel, Sender, Receiver, Select};
use task_async::{TaskManager, Task};

use asynchttp::{miohttp,log};
use asynchttp::miohttp::request::Request;
use asynchttp::miohttp::response::{self, Response};
use asynchttp::miohttp::respchan::Respchan;

use app::api::Request  as apiRequest;
use app::api::Response as apiResponse;

use signal_end::signal_end;



pub fn run_main() {
        
    let addres = "0.0.0.0:2222";
    
    println!("server running - {}", &addres);
    
    let exit_code = run(addres.to_owned());

    // All channels dropped, wait for workers to end.
    //log::debug(format!("Waiting for workers to end..."));

    log::info(format!("Bye."));
    
    process::exit(exit_code);
}


/*

for _ in 0..3 {
    runApp();
}

    lub pętla nieskończona

ctrl-c trzeba obsłużyć

runApp() {
    
    let reset_count = 5;
    
    let (api_prod, api_cons)   = channels();
    let (type_prod, type_cons) = channels();
    
    //...
    
    //uzupełnienie kanału tworzącego
    
    //wrzuć jedno api
    //wrzuć jedno 5 workerów
    
    (new, api)
    (renev, api)
    
    loop {
        match type_cons.get() {
        
            ("new", "api") => {
                run_api("api", api_prod.clone(), api_cons.clone(), type_prod);
                        //jak się wysypie api, to powinien zostać jeszcze raz komunikat wysłany
            }
            
            //inne startowanie
            
            (renew, type) {
                to samo, tylko że zostaje licznik restartów zmniejszony
                    jeśli jest 0, to wychodzimy z procedury
                    
                type_prod.send(()"api", type)
            }
        }
    }
    

}

*/


/*
    TODO - zarządca

    jeśli pod rząd nie może uruchomić programu 3 razy, to fail
    w przypadku gdy poleci panic, to wznawiaj taki proces ...
*/



fn run(addres: String) -> i32 {
    
    let (request_producer, request_consumer) = channel();
    
    log::spawn("<EventLoop>".to_owned(), move ||{
        
                        //grupa tasków
        let task_manager = TaskManager::new(Box::new(move||{

            println!("grupa tasków zakończyłą zadanie");
            //down_producer.send(()).unwrap();
        }));
        
        
        
        let convert = Box::new(move|(req, respchan): (Request, Respchan)| -> (Request, Task<(Response)>) {
            
            let task = task_manager.task(Box::new(move|result : Option<(Response)>|{
                
                match result {
                    
                    Some(resp) => {
                        
                        respchan.send(resp);
                    },
                    
                    None => {
                                                                //coś poszło nie tak z obsługą tego requestu
                        respchan.send(response::Response::create_500());
                    }
                };
                
            }));
            
            (req, task) 
        });
        
        miohttp::server::MyHandler::new(&addres, 4000, 4000, request_producer, convert).unwrap();        
    });
    
    
    
    // Return real OS error to shell, return err.raw_os_error().unwrap_or(-1)
    
    let (api_request_producer , api_request_consumer)  = channel();
    let (api_response_producer, api_response_consumer) = channel();
    
    run_api(&api_request_consumer, &api_response_producer);
    
    
    for _ in 0..4 {
        run_worker(&request_consumer, &api_request_producer, &api_response_consumer);
    }
    
    
    let (sigterm_sender , sigterm_receiver ) = channel();
    let (shutdown_sender, shutdown_receiver) = channel();
    
    signal_end(Box::new(move || {

        log::debug("Termination signal catched.".to_owned());
        
        sigterm_sender.send(()).unwrap();
        
        // oczekuj na zakończenie procedury wyłączania
        let _ = shutdown_receiver.get();
    }));
    
    
    // główna pętla sterująca podwątkami
    loop {
        
        let _ = sigterm_receiver.get();
        
        log::info("Shutting down!".to_owned());
        
        shutdown_sender.send(()).unwrap();
        return 0;
    }
}

fn run_api(api_request_consumer: &Receiver<apiRequest>, api_response_producer: &Sender<apiResponse>) {
    
    let api_request_consumer  = api_request_consumer.clone();
    let api_response_producer = api_response_producer.clone();

    log::spawn("api".to_owned(), move ||{
        api::run(api_request_consumer, api_response_producer);
    });
}

fn run_worker(request_consumer: &Receiver<(Request, Task<(Response)>)>, api_request_producer: &Sender<apiRequest>, api_response_consumer: &Receiver<apiResponse>) {
    
    let request_consumer      = request_consumer.clone();
    let api_request_producer  = api_request_producer.clone();
    let api_response_consumer = api_response_consumer.clone();

    log::spawn("worker".to_owned(), move ||{

        enum Out {
            Result1((Request, Task<(Response)>)),
            Result2(apiResponse),
        }

        let select: Select<Out> = Select::new();

        select.add(request_consumer     , Box::new(Out::Result1));
        select.add(api_response_consumer, Box::new(Out::Result2));

        loop {
            match select.get() {

                Ok(Out::Result1((request, task))) => {

                    worker::render_request(request, task, &api_request_producer);
                },

                Ok(Out::Result2(api::Response::GetFile(result, task))) => {

                    log::debug("Received file data".to_owned());
                    task.result(result);
                },

                Err(_) => {

                    //TODO - zalogować błąd w strumień błędów ... ?
                    return;
                }
            }
        }
    });
}

