use std::io;
use std::path::Path;
use channels_async::Sender;

use asynchttp::log;
use asynchttp::miohttp::{request, response};
use asynchttp::miohttp::response::Response;
use app::api;
use task_async::Task;

pub fn render_request(request: request::Request, task: Task<(Response)>, api_request_producer: &Sender<api::Request>) {
    
    
    let path_src = "./static".to_owned() + request.path().trim();
    log::info(format!("Path requested: {}", &path_src));
    
    
    
    let path = path_src.clone();
    
    let task_get_file = task.async1(Box::new(move|task: Task<(Response)>, response: Option<api::FilesData>|{
        
        log::debug(format!("Invoked request's callback in response"));
        
        match response {
            
            Some(resp_result) => {
                match resp_result {

                    Ok(buffer) => {

                        let buffer = buffer.to_owned();

                        let path         = Path::new(&path_src);
                        let content_type = response::Type::create_from_path(&path);

                        log::info(format!("200, {}, {}", content_type, request.path()));
                        
                        let response = response::Response::create_from_buf(response::Code::Code200, content_type, buffer);

                        task.result(response)
                    }

                    Err(err) => {

                        match err.kind() {

                            io::ErrorKind::NotFound => {

                                let mess     = "Not found".to_owned();
                                let response = response::Response::create(response::Code::Code404, response::Type::TextHtml, mess.clone());
                                log::debug(format!("404, {}, {}. {:?} ", response::Type::TextHtml, request.path(), err));

                                task.result(response)
                            }
                            _ => {

                                log::error(format!("errrrr {:?}", err));
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
