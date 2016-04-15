use std::io;

use channels_async::{Sender, Receiver};
use task_async::{self, Task, callback0};


pub type FilesData = Result<Vec<u8>, io::Error>;


mod get_file;
//pub use self::get_file;

/*
mod render_request;
pub use self::render_request::render_request;
*/



                                                        //TODO - ta zmienna powinna być prywatna
pub enum Request {
    GetFile(String, Task<FilesData>),        //get file content
}



pub struct Api {
    request_chan : Sender<Request>,
}

pub fn run(api_request_producer: Sender<Request>, api_request_consumer: Receiver<Request>, worker_job_producer: Sender<callback0::CallbackBox>) -> (Api, callback0::CallbackBox) {
    
    let start = callback0::new(Box::new(move||{
        
        
                            //TODO - dodać monitoring działania workerów
        
        for _ in 0..5 {

            let api_request_consumer = api_request_consumer.clone();
            let worker_job_producer  = worker_job_producer.clone();
            
            task_async::spawn("api worker".to_owned(), move ||{

                loop {

                    match api_request_consumer.get() {

                        Ok(Request::GetFile(path_src, task)) => get_file::exec(path_src, task, &worker_job_producer),

                        Err(_) => {

                            //TODO - logowanie błędu w strumień błędów ... ?
                            return;
                        }
                    }
                }
            });
        }
    }));
    
    
    let api = Api {
        request_chan : api_request_producer
    };
    
    
    (api, start)
}


impl Api {

    //TODO - dodać parametr będący callbacjuem uruchamianym oi wyłączeniu wszystkich instancji api
    
    
    
    pub fn get_file(&self, path: String, task: Task<FilesData>) {

        self.request_chan.send(Request::GetFile(path, task)).unwrap();
    }

}



impl Clone for Api {
    
    fn clone(&self) -> Api {
        
        Api {
            request_chan : self.request_chan.clone(),
        }
    }
    
    fn clone_from(&mut self, source: &Api) {
        self.request_chan = source.request_chan.clone();
    }
}
