//use miohttp::request;
//use miohttp::response;
use miohttp::log;

use std::io::prelude::Read;
use std::fs::{self, File};
use std::path::Path;
use std::io;

use chan::{Receiver, Sender};
use std::thread;
//use std::time::Duration;

use std::boxed::FnBox;

pub fn run(rx: Receiver<(String, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>, response_data: Sender<(Result<Vec<u8>, io::Error>, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>) {
    
    for _ in 0..2 {
        
        let rx            = rx.clone();
        let response_data = response_data.clone();
        
        thread::spawn(||{
            worker(rx, response_data);
        });
    }
    
    //TODO - dodać monitoring działania workerów
    
}


fn worker(rx: Receiver<(String, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>, response_data: Sender<(Result<Vec<u8>, io::Error>, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>) {

    loop {
        
        match rx.recv() {

            Some((path_str, callback)) => {

                let path = Path::new(&path_str);
                
                println!("plik do odczytania : {:?}", path);

                let response = match fs::metadata(path) {
                    Ok(meta) => {
                        // FIXME: Need to set a limit of max bytes read as na option maybe?
                        if meta.len() > 1_000_000 {
                            log::error(format!("File {:?} is too big to serve", path));
                            Err(io::Error::new(io::ErrorKind::InvalidData, "Static file too big"))
                        } else {
                            match File::open(path) {
                         
                                Ok(mut file) => {
                         
                                    let mut buffer: Vec<u8> = Vec::new();
                         
                                    match file.read_to_end(&mut buffer) {
                                        Ok(_) => Ok(buffer),
                                        Err(err) => Err(err),
                                    }
                                },
                                
                                Err(err) => Err(err),
                            }
                        }
                    }
                    Err(err) => Err(err), 
                };
                
                println!("odpowiedź zwrotna");
                
                response_data.send((response, callback));
            }
            
            None => {
                
                println!("worker statichttp się zakończył");
                return
            }
        }
    }
}
/*
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
}
*/
    
    
/*
//thread::sleep(Duration::new(3, 0));

println!("przetwarzam {:?} {:?}", req, path);

let time_current = time::get_time();
let response_body = format!("Hello user : {} - {}", time_current.sec, time_current.nsec);

let resp = response::Response::create(response::Code::Code200, response::Type::Html, response_body);
let _    = resp_chanel.send((token, resp));
*/

