extern crate miohttp;
extern crate task_async;

use miohttp::{new_server, Request};

mod api_file;
mod server;
mod thread_pool;        //wynieść do zewnętrznego create
mod cache_get;

fn main() {
    println!("server start: 127.0.0.1:9876");

    //Twórz api plików
    //Twórz api workera - zależne od api plików
    
    //create mio (z api workera)
    //create mio (z api workera)
    
    //łap wszystkie sygnały ubitego wątku
    
    let api_file = api_file::ApiFile::new();
    let server = server::Server::new(api_file);
    
    create_mio(server);     //uruchamia i zatrzymuje wątek w tym miejscu
}

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