
//#![feature(plugin)]
//#![plugin(clippy)]

extern crate time;
extern crate channels_async;
extern crate task_async;
extern crate ctrlc;
extern crate miohttp;

mod signal_end;
mod api_file;

mod worker;


use std::process;
use channels_async::{channel, Sender, Receiver, Select};
use task_async::{Task, callback0};
use miohttp::{new_server, Request, Response, Respchan, MioDown};
use api_file::{Api as Api_file, Request as apiRequest};

use signal_end::signal_end;




// #[derive(Debug)]

//TODO - respchan       - trzeba zaimplementować dropa który będzie sprawdzał czy wysłana była odpowiedź, jeśli nie to ma rzucać panic


//TODO - funkcję spawn, można by wsadzić do liba z taskami
    //funkcja spawn powinna współpracować z logowaniem
    //spawn powinno tworzyć ładne "drzewko"
    //natomiast logowanie powinno pozwalać na zgrupowanie logów względem poszczególnych wątków




/*

https://github.com/carllerche/mio/issues/186
https://lwn.net/Articles/542629/

https://github.com/rust-lang-nursery/net2-rs

http://www.unixguide.net/network/socketfaq/4.5.shtml
http://man7.org/linux/man-pages/man7/socket.7.html
https://github.com/tailhook/rotor-http/blob/master/examples/threaded_reuse_port.rs

*/



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


    
    /*
    Counter - ilość działąjących mio,                   --> ilość działających wątków
    
    let count = Counter::new(||{
        //liczba mio spadła do zera
    })
    
    let offMio = newMio(adress, count.clone());     //otwierać współdzielonego socketa
    
    offMio();       //wysyła kanałem informację do eventloopa że ma się on wyłączyć
    
    */
    


/*

let _       = run_mio(&addres, &request_producer);
    
    task_async::spawn("api".to_owned(), move ||{
        
        println!("miodown: będę wyłączał");
        task_async::sleep(5000);
        miodown.shoutdown();
        println!("miodown: wyłączyłem");
    });    
*/



fn main() {
    
    let addres = "0.0.0.0:2222";
    
    println!("server running - {}", &addres);
    
    let exit_code = run(addres.to_owned());
    
    task_async::log_info(format!("Bye."));
    
    process::exit(exit_code);
}



fn run(addres: String) -> i32 {
    
    //TODO - kanały grupa ...
    
    let (request_producer, request_consumer) = channel();
    let (api_request_producer , api_request_consumer)  = channel();
    
    let (worker_job_producer, worker_job_consumer) = channel();
    
    
    
    let miodown = run_mio(&addres, &request_producer);
    
    
    let api_file = run_api(&api_request_producer, &api_request_consumer, &worker_job_producer);
    
    
    for _ in 0..4 {
        
        run_worker(&request_consumer, &api_file, &worker_job_consumer);
    }
    
    
    let sigterm_receiver = install_signal_end();
    
    
    
    // główna pętla sterująca podwątkami
    loop {
        
        let _ = sigterm_receiver.get();
        
        return 0;
    }
}


fn install_signal_end() -> Receiver<()> {
    
    let (sigterm_sender , sigterm_receiver ) = channel();
    
    signal_end(Box::new(move || {
        
        sigterm_sender.send(()).unwrap();
    }));
    
    sigterm_receiver
}


fn run_mio(addres: &String, request_producer: &Sender<(Request, Respchan)>) -> MioDown {
    
    
    let addres           = addres.clone();
    let request_producer = request_producer.clone();
    
    
    let (miodown, miostart) = new_server(addres, 4000, 4000, request_producer);        
    
    
    /*
    Box::new(move||{
        
        println!("grupa tasków zakończyłą zadanie");
        //down_producer.send(()).unwrap();
    })
    */
    
    
    task_async::spawn("<EventLoop>".to_owned(), ||{
        
        miostart.exec();
    });
        
    miodown
}


fn run_api(api_request_producer: &Sender<apiRequest>, api_request_consumer: &Receiver<apiRequest>, worker_job_producer: &Sender<callback0::CallbackBox>) -> api_file::Api {
    
    let api_request_producer = api_request_producer.clone();
    let api_request_consumer = api_request_consumer.clone();
    let worker_job_producer  = worker_job_producer.clone();
    
    let (api, start_api) = api_file::run(api_request_producer, api_request_consumer, worker_job_producer);
    
    task_async::spawn("api".to_owned(), move ||{
        start_api.exec();
    });
    
    api
}


fn run_worker(request_consumer: &Receiver<(Request, Respchan)>, api_file: &Api_file, worker_job_consumer: &Receiver<callback0::CallbackBox>) {
    
    let request_consumer     = request_consumer.clone();
    let api_file             = api_file.clone();
    let worker_job_consumer  = worker_job_consumer.clone();
    
    task_async::spawn("worker".to_owned(), move ||{

        enum Out {
            Result1((Request, Respchan)),
            Result2(callback0::CallbackBox),
        }
        
        let select: Select<Out> = Select::new();

        select.add(request_consumer   , Box::new(Out::Result1));
        select.add(worker_job_consumer, Box::new(Out::Result2));

        loop {
            match select.get() {

                Ok(Out::Result1((request, respchan))) => {
                    
                                                                                        //task gwarantuje drop-a
                    let task = Task::new(Box::new(move|result : Option<(Response)>|{
                        
                        match result {

                            Some(resp) => {
                                
                                respchan.send(resp);
                            },

                            None => {
                                                                        //coś poszło nie tak z obsługą tego requestu
                                respchan.send(Response::create_500());
                            }
                        };
                    }));
                    
                    worker::render_request(&api_file, request, task);
                },
                
                Ok(Out::Result2(job)) => {
                    
                    job.exec();
                },

                Err(_) => {

                    //TODO - zalogować błąd w strumień błędów ... ?
                    return;
                }
            }
        }
    });
}


