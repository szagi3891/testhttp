
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


use channels_async::{channel, Sender, Receiver, Group, Select};
use task_async::{Task, callback0};
use miohttp::{new_server, Request, Response, Respchan, MioDown, MioStart};
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





//kill -INT 1988        - Terminates the program (like Ctrl+C)



fn main() {
    
    let select = run_supervisor();
    
    
    loop {
        match select.get() {

            Ok(_) => {
                //wątek został wyłączony
            },
            Err(_) => {
                
                task_async::log_info(format!("Bye."));
                
                return;
            }
        }
    }
    
}



enum Out {
    Crash(u64),
    Int(()),
}
    

fn run_supervisor() -> Select<Out> {
    
    let addres = "0.0.0.0:2222".to_owned();
    
    println!("server running - {}", &addres);
    
    
    let sigterm_receiver                 = install_signal_end();
    let (crash_producer, crash_consumer) = channel();
    
    
                        //złożenie selecta z sygnału ctrl+c i z sygnału craszu
    let select: Select<Out> = Select::new();

    select.add(sigterm_receiver, Box::new(Out::Int));
    select.add(crash_consumer  , Box::new(Out::Crash));
    
    
    let mut app_counter = 1;
    let mut miodown     = Some(run_app_instance(&addres, &crash_producer, app_counter.clone()));
    
    loop {
        

        match select.get() {

                                                    //sygnał ctrl+c
            Ok(Out::Int(())) => {
                
                match mem::replace(&mut miodown, None) {
                    Some(down) => {
                        
                        task_async::log_info("signal INT".to_owned());
                        task_async::log_info(format!("miodown, app_id={}", app_counter));
                        
                        down.shoutdown();
                    },
                    None => {},
                }
                
                return select;
            },

                                                    //padł wątek w instancji aplikacji
            Ok(Out::Crash(app_id)) => {
                
                if app_counter == app_id && miodown.is_some() {
                    
                    app_counter = app_counter + 1;
                    
                    task_async::log_info(format!("restart app, new app_id={}", app_counter));
                    
                    let miodown2 = run_app_instance(&addres, &crash_producer, app_counter);
                    
                    match mem::replace(&mut miodown, Some(miodown2)) {
                        Some(down) => down.shoutdown(),
                        None => {},
                    }
                }
            },

            Err(_) => {
                
                return select;
            }
        }
    }
}





fn run_app_instance(addres: &String, crash_producer: &Sender<u64>, current_app_counter: u64) -> MioDown {
    
    
    let mut channel_group = Group::new();
    
    
                                                                //channel_group.channel::<callback0::CallbackBox>();
    let (job_producer, job_consumer) = channel_group.channel();
    
    
    let (api_file, start_api) = api_file::create(&mut channel_group, &job_producer);
    
    {
        let crash_producer = crash_producer.clone();
        let current_app_counter = current_app_counter.clone();
        
        task_async::spawn_defer("<api>".to_owned(), move ||{
            
            start_api.exec();
            
        }, move||{
            
            task_async::log_info("down".to_owned());
            crash_producer.send(current_app_counter).unwrap();
        });
    }
    
    
    
    
    let (miostart, miodown) = run_mio(&addres, &api_file, &job_producer);
    
    {
        let crash_producer = crash_producer.clone();
        let current_app_counter = current_app_counter.clone();
        
        task_async::spawn_defer("<EventLoop>".to_owned(), move||{
            
            miostart.start();
        
        }, move||{

            task_async::log_info("down".to_owned());
            crash_producer.send(current_app_counter).unwrap();
            channel_group.close();
        });
    }
    
    
    
    for _ in 0..4 {
        
        let start_worker = run_worker(&job_consumer);
        
        let crash_producer = crash_producer.clone();
        let current_app_counter = current_app_counter.clone();
        
        task_async::spawn_defer("<worker>".to_owned(), move ||{
            
            start_worker.exec();
            
        }, move||{

            task_async::log_info("down".to_owned());
            crash_producer.send(current_app_counter).unwrap();
        });
    }
    
    miodown
}




fn install_signal_end() -> Receiver<()> {
    
    let (sigterm_sender , sigterm_receiver) = channel();
    
    
    task_async::spawn("<sigterm>".to_owned(), move ||{
        
        signal_end(callback0::new(Box::new(move||{

            sigterm_sender.send(()).unwrap();
        })));
    });
    
    sigterm_receiver
}


fn run_mio(addres: &String, api_file: &Api_file, job_producer: &Sender<callback0::CallbackBox>) -> (MioStart, MioDown) {
    
    let addres       = addres.clone();
    let api_file     = api_file.clone();
    let job_producer = job_producer.clone();
    
        
    
    
    let log_error = Box::new(|is_error : bool, message:String|{
        
        if is_error {
            
            println!("ERROR: {}", message);
            
        } else {
            
            println!("LOG  : {}", message);
        }
    });
    
        
    new_server(addres, 4000, 4000, Some(log_error), Box::new(move|(request, respchan):(Request, Respchan)| {
        
        let api_file = api_file.clone();
        
        job_producer.send(callback0::new(Box::new(move||{
            
                                                                   //task gwarantuje drop-a

            worker::render_request(api_file, request, Task::new(Box::new(move|result : Option<(Response)>|{

                match result {

                    Some(resp) => {

                        respchan.send(resp);
                    },

                    None => {
                                                                //coś poszło nie tak z obsługą tego requestu
                        respchan.send(Response::create_500());
                    }
                };
            })));
        
        }))).unwrap();
    }))
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


