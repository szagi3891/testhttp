
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
    
    let exit_code = run_supervisor();
    
    task_async::log_info(format!("Bye."));
    
    process::exit(exit_code);
}



enum Out {
    Crash(u64),
    Int(()),
}
    

fn run_supervisor() -> i32 {
    
    let addres = "0.0.0.0:2222".to_owned();
    
    println!("server running - {}", &addres);
    
    
    let sigterm_receiver = install_signal_end();
    
    
    let (exit_code_producer , exit_code_consumer)  = channel();
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
    
    
    let app_counter = 1;
    let miodown     = make_app(app_counter.clone());
    
    
    run_waiting(app_counter, Some(miodown), select, make_app, 0, exit_code_producer);
    
    
    match exit_code_consumer.get() {
        
        Ok(code) => code,
        Err(_) => 1,
    }
}


fn run_waiting(current_app_id: u64, miodown_opt: Option<MioDown>, select: Select<Out>, make_app: Box<Fn(u64) -> MioDown + Send + Sync + 'static>, exit_code: i32, exit_code_producer: Sender<i32>) {

    task_async::spawn("<supervisor>".to_owned(), move ||{
        
        match select.get() {

                                                    //sygnał ctrl+c
            Ok(Out::Int(())) => {
                
                println!("waiting: int");
                
                if let Some(miodown) = miodown_opt {
                    miodown.shoutdown();
                }
                
                
                task_async::spawn("<sdown>".to_owned(), move ||{
                    loop {
                        match select.get() {
                            Ok(_) => {
                                println!("pad 1 ...");
                            },
                            Err(_) => {
                                
                                println!("pad 2 ...");
                                
                                exit_code_producer.send(0).unwrap();
                                return;
                            }
                        }
                    }
                });
                
                return;
            },

                                                    //padł wątek w instancji aplikacji
            Ok(Out::Crash(app_id)) => {
                
                if current_app_id == app_id {
                    
                    if let Some(miodown) = miodown_opt {

                        let miodown2 = make_app(app_id + 1);

                        miodown.shoutdown();

                        run_waiting(app_id + 1, Some(miodown2), select, make_app, 0, exit_code_producer);
                        return;
                    }
                }
                
                run_waiting(app_id, None, select, make_app, 0, exit_code_producer);
            },

            Err(_) => {
                
                exit_code_producer.send(0).unwrap();
                return;
            }
        }
    });
}





fn run_app_instance(addres: &String, crash_chan_producer: &Sender<u64>, current_app_counter: u64) -> MioDown {
    
    
    let mut channel_group = Group::new();
    
    
    let (job_producer, job_consumer) = channel_group.channel();
    
    
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
    
    
    
    
    //TODO - dorobić funkcję : task_async::spawn_defer(move||{ ... }, move||{ ... })
    
    
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
        
        task_async::sleep(5000);
        sigterm_sender.send(()).unwrap();
        
        /*
        println!("odpalam 000");
        
        signal_end(callback0::new(Box::new(move||{
            
            
            let _defer = Defer::new(callback0::new(Box::new(move||{
                
                task_async::log_info("defer signal_end".to_owned());
            })));
            
            
            println!("odpalam 111");
            
            sigterm_sender.send(()).unwrap();
            
            println!("odpalam 222");
        })));
        */
    });
    
    
    sigterm_receiver
    
    /*
    println!("odpalam ... sigterm");
    
    
    task_async::spawn("<sigterm>".to_owned(), move ||{
        
        //task_async::sleep(5000);
        //sigterm_sender.send(()).unwrap();
        
        println!("odpalam 000");
        
        signal_end(Box::new(move || {
            
            println!("odpalam 111");
            sigterm_sender.send(()).unwrap();
            println!("odpalam 222");
        }));
    });
    
    sigterm_receiver
    */
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


