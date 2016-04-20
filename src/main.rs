
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
    
    
    let sigterm_receiver = install_signal_end();
    
    
    let (crash_chan_producer, crash_chan_consumer) = channel();
    
    
    let make_app = {
        
        let addres              = addres.clone();
        let crash_chan_producer = crash_chan_producer.clone();
        
        Box::new(move|app_counter: u64| -> MioDown {
            run_app_instance(&addres, &crash_chan_producer, app_counter)
        })
    };
    
    
    //złożenie selecta z sygnału ctrl+c i z sygnału craszu
    
    
    
    let select: Select<Out> = Select::new();

    select.add(sigterm_receiver   , Box::new(Out::Int));
    select.add(crash_chan_consumer, Box::new(Out::Crash));
    
    
    let mut app_counter = 1;
    let mut miodown     = Some(make_app(app_counter.clone()));
    
    
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
                    
                    let miodown2 = make_app(app_counter);

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





fn run_app_instance(addres: &String, crash_chan_producer: &Sender<u64>, current_app_counter: u64) -> MioDown {
    
    
    let mut channel_group = Group::new();
    
    
    let (job_producer, job_consumer) = channel_group.channel::<callback0::CallbackBox>();
    
    
    let (api_file, start_api) = api_file::create(&mut channel_group, &job_producer);
    
    {
        let crash_chan_producer = crash_chan_producer.clone();
        let current_app_counter = current_app_counter.clone();
        
        task_async::spawn("<api>".to_owned(), move ||{

            let _defer = Defer::new(callback0::new(Box::new(move||{

                task_async::log_info("down".to_owned());
                crash_chan_producer.send(current_app_counter).unwrap();
            })));

            start_api.exec();
        });
    }
    
    
    
    
    let (miodown, miostart) = run_mio(&addres, &api_file, &job_producer);
    
    {
        let crash_chan_producer = crash_chan_producer.clone();
        let current_app_counter = current_app_counter.clone();
        
        task_async::spawn("<EventLoop>".to_owned(), move||{

            let _defer = Defer::new(callback0::new(Box::new(move||{

                task_async::log_info("down".to_owned());
                channel_group.close();
                crash_chan_producer.send(current_app_counter).unwrap();
            })));

            miostart.exec();
        });
    }
    
    
    
    
    //TODO - dorobić funkcję : task_async::spawn_defer(move||{ ... }, move||{ ... }) -- ?
    
    
    for _ in 0..4 {
        
        let start_worker = run_worker(&job_consumer);
        
        let crash_chan_producer = crash_chan_producer.clone();
        let current_app_counter = current_app_counter.clone();
        
        task_async::spawn("<worker>".to_owned(), move ||{
            
            let _defer = Defer::new(callback0::new(Box::new(move||{

                task_async::log_info("down".to_owned());
                crash_chan_producer.send(current_app_counter).unwrap();
            })));
            
            start_worker.exec();
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


