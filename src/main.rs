
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
use channels_async::{channel, Sender, Receiver, Group};
use task_async::{Task, callback0};
use miohttp::{new_server, Request, Response, Respchan, MioDown};
use api_file::{Api as Api_file};

use signal_end::signal_end;

use std::mem;

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



//http://stackoverflow.com/questions/29963449/golang-like-defer-in-rust

struct Defer {
    func: callback0::CallbackBox
}

impl Drop for Defer {
    
    fn drop(&mut self) {
        
        let empty_clouser = callback0::new(Box::new(||{}));
        let func = mem::replace(&mut self.func, empty_clouser);
        
		func.exec();
    }
}

impl Defer {
    
    fn new(func : callback0::CallbackBox) -> Defer {
        Defer {
            func : func
        }
    }
}





fn main() {
    
    let exit_code = run_main();
    
    process::exit(exit_code);
}

fn run_main() -> i32 {
    
    let exit_code = run_supervisor();
    
    task_async::log_info(format!("Bye."));
    
    exit_code
}



fn run_supervisor() -> i32 {
    
    let addres = "0.0.0.0:2222";
    
    println!("server running - {}", &addres);
    
    
    let sigterm_receiver = install_signal_end();
    
    let (crash_chan_producer, crash_chan_consumer) = channel();
    
    
    
                                        //odpalenie całęgo drzewa procesów
        
    let mut miodown = Some(run_app_instance(addres.to_owned(), crash_chan_producer.clone()));
    
    
                            //TODO - temp
                            let _ = sigterm_receiver.get();
    
    
    
    
    
    //loop {
        
        
        /*
        
        miodown = obserwe_crash(miodown);
        
        
        if miodown.is_nond() {
                                //czekaj aż wszystkie wątki wyparują
            loop {
            
                match crach_chan.get() {
                    Err(_) => return 1;     //kanał niezdatny do odczytu - wtedy dopiero można spokojnie zakończyć żywot
                    Ok(_) => {}             //czekaj dalej
                }
            }
        }
        
        
        
        fn obserwe_crash(miodown) {
                                let sigterm = sigterm_receiver.get();
            //trzeba skeić selecta
    
            select {
                sigterm -> {

                    miodown.exec();

                    None
                }

                obserwator_padu -> {

                    let miodown2 = run_app_instance(addres.to_owned(), kanał informujący o awariach nadawca)

                    miodown.exec();

                    miodown2
                }
            }
        }
        */
        
        
    //}
    
    0
}


fn run_app_instance(addres: String, crash_chan_producer: Sender<()>) -> MioDown {
    
    
    //drugi kanał kanał, on będzie zwracał informację o padniętym wątku
        //zwróci ten kanał jako drugą wartość tej tupli
        //ten kanał przekazujemy do wszystkich podwątków
            //instalowane wysłanie wartości będzie na span, z użyciem defer ...
            
    
    
    let mut channel_group = Group::new();
    
    
    let (job_producer, job_consumer) = channel_group.channel();
    
    
    
    let (api_file, start_api) = api_file::create(&mut channel_group, &job_producer);
    
    task_async::spawn("api".to_owned(), move ||{
        
        let _defer = Defer::new(callback0::new(Box::new(move||{
            
            println!("defer1");
        })));
        
        start_api.exec();
    });
    
    
    
    
    let crash_chan_producer = crash_chan_producer.clone();
    
    let (miodown, miostart) = run_mio(&addres, &api_file, &job_producer);
    
    task_async::spawn("<EventLoop>".to_owned(), ||{
        
        let _defer = Defer::new(callback0::new(Box::new(move||{

            println!("defer2");
        })));
        
        miostart.exec();
    });
    
    
    
    
    
    
    //run_mio(&addres, &api_file, &job_producer, callback
        //callback będzie uruchamiany w momencie gdy mio obsłyżył już wszystkie połączenia
        //w tym momencie odpalamy zamykanie grupy
    
    
    for _ in 0..4 {
        
        let start_worker = run_worker(&job_consumer);
        
        task_async::spawn("worker".to_owned(), move ||{
            
            let _defer = Defer::new(callback0::new(Box::new(move||{

                println!("defer3");
            })));
            
            start_worker.exec();
        });
    }
    
    miodown
}


fn install_signal_end() -> Receiver<()> {
    
    let (sigterm_sender , sigterm_receiver ) = channel();
    
    signal_end(Box::new(move || {
        
        sigterm_sender.send(()).unwrap();
    }));
    
    sigterm_receiver
}


fn run_mio(addres: &String, api_file: &Api_file, job_producer: &Sender<callback0::CallbackBox>) -> (MioDown, callback0::CallbackBox) {
    
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
    
    
    new_server(addres, 4000, 4000, job_producer, convert)
}



fn run_worker(job_consumer: &Receiver<callback0::CallbackBox>) -> callback0::CallbackBox {
    
    let job_consumer = job_consumer.clone();
    
    let start = callback0::new(Box::new(move ||{
        
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
    }));
    
    start
}


