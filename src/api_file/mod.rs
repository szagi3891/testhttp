use std::io;
//use std::collections::HashMap;
use channels_async::{Group, Sender};
use task_async::{self, Task, callback0};


mod get_file;


pub type FilesData = Result<Vec<u8>, io::Error>;

pub enum Request {
    GetFile(String, Task<FilesData>),        //get file content
}

pub enum Response {
    GetFile(String, Task<FilesData>, FilesData),                //TODO - informacja o tasku ma wylecieć
}


pub enum ChanType {
    Input(Request),
    Subworker(Response),
}



pub struct Api {
    request_chan : Sender<ChanType>,
}

pub fn create(channel_group: &mut Group, job_producer: &Sender<callback0::CallbackBox>) -> (Api, callback0::CallbackBox) {
    
    
    let (api_producer, api_consumer)       = channel_group.channel();
    let (worker_producer, worker_consumer) = channel_group.channel();
    
    let job_producer = job_producer.clone();
    
    
    let api = Api {
        request_chan : api_producer.clone(),
    };
    
    
    let start = callback0::new(Box::new(move||{
        
        
                            //TODO - dodać monitoring działania workerów
        
        for _ in 0..5 {
            
            let api_producer    = api_producer.clone();
            let worker_consumer = worker_consumer.clone();
            
            task_async::spawn("api worker".to_owned(), move ||{
                
                loop {

                    match worker_consumer.get() {

                        Ok(Request::GetFile(path_src, task)) => get_file::exec(path_src, task, &api_producer),
                        
                        Err(_) => {

                            //TODO - logowanie błędu w strumień błędów ... ?
                            return;
                        }
                    }
                }
            });
        }
        
        
        //TODO - dorobić cache
        //let cache = HashMap::new();     //HashMap<String, FilesData>,
        
        
        loop {
            
            match api_consumer.get() {
                
                Ok(ChanType::Input(Request::GetFile(path_src, task))) => {
                    
                    //TODO - tutaj dorobić korzystanie z kesza
                    
                    worker_producer.send(Request::GetFile(path_src, task)).unwrap();
                },
                
                Ok(ChanType::Subworker(Response::GetFile(path_src, task, files_data))) => {
                    
                    let job = callback0::new(Box::new(move||{
                        task.result(files_data)
                    }));

                    job_producer.send(job).unwrap();
                },
                
                Err(_) => {
                    
                    //TODO - logowanie błędu ?
                    return;
                }
            }
        }                
        
    }));
    
    
    (api, start)
}


impl Api {

    //TODO - dodać parametr będący callbacjuem uruchamianym oi wyłączeniu wszystkich instancji api
    
    
    
    pub fn get_file(&self, path: String, task: Task<FilesData>) {

        self.request_chan.send(ChanType::Input(Request::GetFile(path, task))).unwrap();
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
