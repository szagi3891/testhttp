mod api;
mod worker;

use std::process;
use channels_async::{channel, Sender, Receiver, Select};
use task_async::{TaskManager, Task};

use asynchttp::{miohttp,log};
use asynchttp::miohttp::request::Request;
use asynchttp::miohttp::response::Response;

use app::api::Request  as apiRequest;
use app::api::Response as apiResponse;

use signal_end::signal_end;

use std::thread;

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


//TODO - spawn, ma za zadanie robić sprytne nazwy wątków


fn run(addres: String) -> i32 {
    
    let (request_producer, request_consumer) = channel();

    let thread_name = "<EventLoop>".to_owned();

    spawn(thread_name, move ||{
        
                        //grupa tasków
        let task_manager = TaskManager::new(Box::new(move||{

            println!("grupa tasków zakończyłą zadanie");
            //down_producer.send(()).unwrap();
        }));
        
        
        //TODO - ogólnie, do dalszego przetwarzania będzie wysyłana para, (request, task)
        
        miohttp::server::MyHandler::new(&addres, 4000, 4000, request_producer, Box::new(|req:Request|->(Request, Task) {
            
            let task = task_manager.task(Box::new(move|result : Option<(Request, Response)>|{
                
                match result {
                    Some((req, resp)) => req.resp(resp),
                    None => {
                        
                        //coś poszło nie tak z obsługą tego requestu
                    }
                };
                
            }));
            
            (req, task)
            
        })).unwrap();
        
        //funkcja, przetwarzająca request na nowy rodzaj typu
        
        
    });
    
    
    
    // Return real OS error to shell, return err.raw_os_error().unwrap_or(-1)
    
    let (api_request_producer , api_request_consumer)  = channel();
    let (api_response_producer, api_response_consumer) = channel();
    
    {
        
        let api_request_consumer  = api_request_consumer.clone();
        let api_response_producer = api_response_producer.clone();

        spawn("api".to_owned(), move ||{
            api::run(api_request_consumer, api_response_producer);
        });
    }
    
    
    
    for _ in 0..4 {
        
        let request_consumer      = request_consumer.clone();
        let api_request_producer  = api_request_producer.clone();
        let api_response_consumer = api_response_consumer.clone();
        
        spawn("worker".to_owned(), move ||{
            run_worker(request_consumer, api_request_producer, api_response_consumer);
        });
    }
    
    
    let (sigterm_sender,  sigterm_receiver ) = channel();
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

//TODO - ubibliotecznić to sprytnie
pub fn spawn<F, T>(name: String, block: F)
    where F: FnOnce() -> T + Send + Sync + 'static, T: Send + Sync + 'static {

    
    let result = thread::Builder::new().name(name.clone()).spawn(block);
        
    match result {
        Ok(_) => {},
        Err(err) => panic!("Can't spawn {}: {}", name, err),
    };
}


/*
let _ = Manager::new("api".to_owned(), 1, Box::new(move|thread_name: String|{
}));
*/

fn run_worker(request_consumer: Receiver<Request>, api_request_producer: Sender<apiRequest>, api_response_consumer: Receiver<apiResponse>) {
    
    enum Out {
        Result1(Request),
        Result2(apiResponse),
    }
    
    let select: Select<Out> = Select::new();
    
    select.add(request_consumer     , Box::new(Out::Result1));
    select.add(api_response_consumer, Box::new(Out::Result2));
    
    loop {
        match select.get() {
            Ok(Out::Result1(request)) => {
                worker::render_request(request, &api_request_producer);
            },
            Ok(Out::Result2(api::Response::GetFile(result, callback))) => {
                log::debug("Received file data".to_owned());
                callback.call_box((result,));
            },
            Err(_) => {
                
                //TODO - zalogować błąd w strumień błędów ... ?
                return;
            }
        }
    }
}


