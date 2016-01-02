use mio;
use miohttp::request;
use miohttp::response;

use std::io::prelude::Read;
use std::fs::File;
use std::path::Path;
use std::io;

use std::thread;
//use std::time::Duration;

pub fn process_request(req: request::Request, token: mio::Token, resp_chanel: mio::Sender<(mio::Token, response::Response)>) {
    
    thread::spawn(move || {
        
        let path_str = "./static".to_string() + req.path.trim();
        let path     = Path::new(&path_str);
        
        //TODO - trzeba zabezpieczyć żeby requesty nie mogły wychodzić poza główny katalog
        
        match File::open(path) {
            
            Ok(mut file) => {
                
                let mut buffer: Vec<u8> = Vec::new();
                
                match file.read_to_end(&mut buffer) {
                    
                    Ok(_) => {
                        
                        //TODO - trzeba wyznaczyć rozszerzenie na podstawie ścieżki i na jego podstawie wybrać odpowiedni mime
                        
                        let resp = response::Response::create_from_buf(response::Code::Code200, response::Type::Html, buffer);
                        let _    = resp_chanel.send((token, resp));
                    }
                    
                    Err(err) => {
                        println!("errrrr {:?}", err);
                    }
                }
            }
            
            Err(err) => {
                
                match err.kind() {
                    
                    io::ErrorKind::NotFound => {
                        
                        let mess =  "Not fund".to_string();
                        let resp = response::Response::create(response::Code::Code404, response::Type::Html, mess);
                        let _    = resp_chanel.send((token, resp));
                    }
                    _ => {
                        println!("errrrr {:?}", err);
                        
                        //TODO - trzeba zaimplementować drop w request, który automatycznie stworzy odpowiedź 500 i wyśle ją do mio
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
