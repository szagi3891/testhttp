use miohttp::request;
use miohttp::response;

use std::io::prelude::Read;
use std::fs::File;
use std::path::Path;
use std::io;

use std::thread;
//use std::time::Duration;

pub fn process_request(request: request::Request) {
    
    thread::spawn(move || {
        
        let path_str = "./static".to_string() + request.path.trim();
        let path     = Path::new(&path_str);
        
        //TODO - trzeba zabezpieczyć żeby requesty nie mogły wychodzić poza główny katalog
        
        match File::open(path) {
            
            Ok(mut file) => {
                
                let mut buffer: Vec<u8> = Vec::new();
                
                match file.read_to_end(&mut buffer) {
                    
                    Ok(_) => {
                                                
                        //TODO - trzeba wyznaczyć rozszerzenie na podstawie ścieżki i na jego podstawie wybrać odpowiedni mime
                        //https://doc.rust-lang.org/std/path/struct.Path.html#method.extension
                        
                        let response = response::Response::create_from_buf(response::Code::Code200, response::Type::Html, buffer);
                        
                        request.send(response);
                    }
                    
                    Err(err) => {
                        println!("errrrr {:?}", err);
                    }
                }
            }
            
            Err(err) => {
                
                match err.kind() {
                    
                    io::ErrorKind::NotFound => {
                        
                        let mess     = "Not fund".to_string();
                        let response = response::Response::create(response::Code::Code404, response::Type::Html, mess);
                        request.send(response);
                    }
                    _ => {
                        
                        println!("errrrr {:?}", err);
                    }
                }
            }
        }
    });
    
    
    /*
    //thread::sleep(Duration::new(3, 0));
    
    println!("przetwarzam {:?} {:?}", req, path);

    let time_current = time::get_time();
    let response_body = format!("Hello user : {} - {}", time_current.sec, time_current.nsec);

    let resp = response::Response::create(response::Code::Code200, response::Type::Html, response_body);
    let _    = resp_chanel.send((token, resp));
    */

}
