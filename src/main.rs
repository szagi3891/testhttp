
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
use channels_async::{channel, Sender, Receiver};
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

//request_consumer: &Receiver<(Request, Respchan)>, 


fn run(addres: String) -> i32 {
    
    //TODO - kanały grupa ...
    
    
    let (api_request_producer , api_request_consumer)  = channel();
    
    let (job_producer, job_consumer) = channel();
    
    
    
    let api_file = run_api(&api_request_producer, &api_request_consumer, &job_producer);
    
    
    let miodown = run_mio(&addres, &api_file, &job_producer);
    
    
    
    for _ in 0..4 {
        
        run_worker(&job_consumer);
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


fn run_mio(addres: &String, api_file: &Api_file, job_producer: &Sender<callback0::CallbackBox>) -> MioDown {
    
    let addres       = addres.clone();
    let api_file     = api_file.clone();
    let job_producer = job_producer.clone();
    
    let convert = Box::new(move|(request, respchan):(Request, Respchan)| -> callback0::CallbackBox {
        
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
        
        
        let api_file = api_file.clone();
        
        callback0::new(Box::new(move||{
            
            worker::render_request(api_file, request, task);
        }))
    });
    
    
    let (miodown, miostart) = new_server(addres, 4000, 4000, job_producer, convert);        
    
    
    task_async::spawn("<EventLoop>".to_owned(), ||{
        
        miostart.exec();
    });
        
    miodown
    
    
    /*
    let convert = Box::new(move|(request, respchan):(Request, Respchan)| -> callback0::CallbackBox {

        
        callback0::new(Box::new(move||{
            
            worker::render_request(&api_file, request, task);
        }))
    });
    */
    
}


fn run_api(api_request_producer: &Sender<apiRequest>, api_request_consumer: &Receiver<apiRequest>, job_producer: &Sender<callback0::CallbackBox>) -> api_file::Api {
    
    let api_request_producer = api_request_producer.clone();
    let api_request_consumer = api_request_consumer.clone();
    let job_producer  = job_producer.clone();
    
    let (api, start_api) = api_file::run(api_request_producer, api_request_consumer, job_producer);
    
    task_async::spawn("api".to_owned(), move ||{
        start_api.exec();
    });
    
    api
}


fn run_worker(job_consumer: &Receiver<callback0::CallbackBox>) {
    
    let job_consumer = job_consumer.clone();
    
    task_async::spawn("worker".to_owned(), move ||{
        
        loop {
            match job_consumer.get() {

                Ok(job) => {
                    
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


