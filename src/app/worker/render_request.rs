use std::io;
use std::path::Path;
use channels_async::Sender;

use miohttp::{Request, Response, Type, Code};
use app::api;
use task_async::Task;
use task_async;


pub fn render_request(request: Request, task: Task<(Response)>, api_request_producer: &Sender<api::Request>) {
    
    
    let path_src = "./static".to_owned() + request.path().trim();
    task_async::log_info(format!("Path requested: {}", &path_src));
    
    
    
    let path = path_src.clone();
    
    let task_get_file = task.async1(Box::new(move|task: Task<(Response)>, response: Option<api::FilesData>|{
        
        task_async::log_debug(format!("Invoked request's callback in response"));
        
        match response {
            
            Some(resp_result) => {
                match resp_result {

                    Ok(buffer) => {

                        let buffer = buffer.to_owned();

                        let path         = Path::new(&path_src);
                        let content_type = Type::create_from_path(&path);

                        task_async::log_info(format!("200, {}, {}", content_type, request.path()));
                        
                        let response = Response::create_from_buf(Code::Code200, content_type, buffer);

                        task.result(response)
                    }

                    Err(err) => {

                        match err.kind() {

                            io::ErrorKind::NotFound => {

                                let mess     = "Not found".to_owned();
                                let response = Response::create(Code::Code404, Type::TextHtml, mess.clone());
                                task_async::log_debug(format!("404, {}, {}. {:?} ", Type::TextHtml, request.path(), err));

                                task.result(response)
                            }
                            _ => {

                                task_async::log_error(format!("errrrr {:?}", err));
                            }
                        }

                    }
                }
            }
            
            None => {
                //api nie odpowiedziało
            }
        }
    }));
    
    api_request_producer.send(api::Request::GetFile(path, task_get_file)).unwrap();
}
