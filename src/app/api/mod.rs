use std::io::prelude::Read;
use std::fs::{self, File};
use std::path::Path;
use std::io;

use channels_async::{Sender, Receiver};

use asynchttp::log;
use asynchttp::async::{spawn, Callback, Manager};


pub type FilesData  = Result<Vec<u8>, io::Error>;
pub type CallbackFD = Callback<FilesData>;


pub enum Request {
    GetFile(String, CallbackFD),        //get file content
}

pub enum Response {
    GetFile(FilesData, CallbackFD),     //get file content
}



pub fn run(api_request_consumer: Receiver<Request>, api_response_producer: Sender<Response>) {

    let manager_static_workers = Manager::new("static worker".to_owned(), 5, Box::new(move|thread_name| {
        
        let api_request_consumer  = api_request_consumer.clone();
        let api_response_producer = api_response_producer.clone();
        
        match spawn(thread_name.to_owned(), move ||{
            worker(api_request_consumer, api_response_producer);
        }) {
                Ok(join_handle) => join_handle,
                Err(err) => panic!("Can't spawn {}: {}", thread_name, err),
        };
    }));

    //TODO - dodać monitoring działania workerów
}


fn worker(rx_api_request: Receiver<Request>, tx_api_response: Sender<Response>) {

    loop {
        
        match rx_api_request.get() {

            Request::GetFile(path_src, callback) => {
                
                get_file(path_src, callback, &tx_api_response);
            }
        }
    }
}


fn get_file(path_src: String, callback: CallbackFD, tx_api_response: &Sender<Response>) {
    
    let path = Path::new(&path_src);

    log::debug(format!("Loading file {:?}", path));

    let response = match fs::metadata(path) {
        Ok(meta) => {
            // FIXME: Need to set a limit of max bytes read as na option maybe?
            if meta.len() > 1_000_000 {
                log::error(format!("File {:?} is too big to serve", path));
                Err(io::Error::new(io::ErrorKind::InvalidData, "Static file too big"))
            } else {
                match File::open(path) {

                    Ok(mut file) => {

                        let mut file_data: Vec<u8> = Vec::new();

                        match file.read_to_end(&mut file_data) {
                            Ok(_) => {
                                log::debug(format!("Sending response ({} bytes).", file_data.len()));
                                Ok(file_data)
                            }
                            Err(err) => Err(err),
                        }
                    },

                    Err(err) => Err(err),
                }
            }
        }
        Err(err) => Err(err), 
    };

    tx_api_response.send(Response::GetFile(response, callback));
    log::debug(format!("Response sent."));
}


/*
pub fn process_request(request: request::Request) {
    
    thread::spawn(move || {
        
        let path_str = "./static".to_owned() + request.path.trim();
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
                        
                        let mess     = "Not fund".to_owned();
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

