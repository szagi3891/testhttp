extern crate miohttp;
extern crate task_async;

use miohttp::{new_server, Request};
use task_async::{TaskPool};

mod api_file;
mod server;
mod cache_get;

fn main() {
    println!("server start: 127.0.0.1:9876");

    //Twórz api plików
    //Twórz api workera - zależne od api plików
    
    //create mio (z api workera)
    //create mio (z api workera)
    
    //łap wszystkie sygnały ubitego wątku
    
    let api_file = api_file::ApiFile::new(5);
    let task_pool = TaskPool::new(4);
    //let cache_file = cache_file::cache_file::new(api_file);
    let server = server::Server::new(task_pool, api_file);
    
    create_mio(server);     //uruchamia i zatrzymuje wątek w tym miejscu
}

/*

use std::time::Duration;
use callback0;
use std::mem;

            //zrobić z tego bibliotekę do zarządzania procesami

//TODO - ubibliotecznić to sprytnie
pub fn spawn<F, T>(name: String, block: F)
    where F: FnOnce() -> T + Send + Sync + 'static, T: Send + Sync + 'static {

    
    let result = thread::Builder::new().name(name.clone()).spawn(block);
        
    match result {
        Ok(_) => {},
        Err(err) => panic!("Can't spawn {}: {}", name, err),
    };
}



pub fn spawn_defer<F, T, D>(name: String, block: F, defer: D)
    where
        F: FnOnce() -> T + Send + Sync + 'static,
        T: Send + Sync + 'static,
        D: FnOnce() + Send + Sync + 'static {

    spawn(name, move||{
        
        let _defer = Defer::new(callback0::new(Box::new(defer)));
        
        block()
    });
}


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



pub fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}
*/

fn create_mio(server: server::Server) {

    let log_error = Box::new(|is_error : bool, message:String|{
        
        if is_error {
            
            println!("ERROR: {}", message);
            
        } else {
            
            println!("LOG  : {}", message);
        }
    });
    
    
    let addres = "127.0.0.1:9876".to_owned();
    
    new_server(addres, 4000, 4000, Some(log_error), Box::new(move|request:Request| {
        
        server.process(request);
    }));
}
