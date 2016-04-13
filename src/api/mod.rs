use std::io::prelude::Read;
use std::fs::{self, File};
use std::path::Path;
use std::io;

use channels_async::{Sender, Receiver};
use task_async::Task;
use task_async;

pub type FilesData   = Result<Vec<u8>, io::Error>;

//TODO - odwrócić kolejność, najpierw task

pub enum Request {
    GetFile(String, Task<FilesData>),        //get file content
}

pub enum Response {
    GetFile(FilesData, Task<FilesData>),     //get file content
}



pub fn run(api_request_consumer: Receiver<Request>, api_response_producer: Sender<Response>) {

    for _ in 0..5 {
        
        let api_request_consumer  = api_request_consumer.clone();
        let api_response_producer = api_response_producer.clone();
        
        task_async::spawn("api worker".to_owned(), move ||{
            worker(api_request_consumer, api_response_producer);
        });
    }

    //TODO - dodać monitoring działania workerów
}

fn worker(rx_api_request: Receiver<Request>, tx_api_response: Sender<Response>) {

    loop {
        
        match rx_api_request.get() {

            Ok(Request::GetFile(path_src, callback)) => {
                
                get_file(path_src, callback, &tx_api_response);
            }
            Err(_) => {
                
                //TODO - logowanie błędu w strumień błędów ... ?
                return;
            }
        }
    }
}


fn get_file(path_src: String, task: Task<FilesData>, tx_api_response: &Sender<Response>) {
    
    let path = Path::new(&path_src);

    task_async::log_debug(format!("Loading file {:?}", path));

    let response = match fs::metadata(path) {
        Ok(meta) => {
            
            // FIXME: Need to set a limit of max bytes read as na option maybe?
            if meta.len() > 1_000_000 {
                task_async::log_error(format!("File {:?} is too big to serve", path));
                Err(io::Error::new(io::ErrorKind::InvalidData, "Static file too big"))
            } else {
                match File::open(path) {

                    Ok(mut file) => {

                        let mut file_data: Vec<u8> = Vec::new();

                        match file.read_to_end(&mut file_data) {
                            Ok(_) => {
                                task_async::log_debug(format!("Sending response ({} bytes).", file_data.len()));
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

    tx_api_response.send(Response::GetFile(response, task)).unwrap();
    task_async::log_debug(format!("Response sent."));
}

