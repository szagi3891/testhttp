use std::io;
use std::path::Path;
use api_file::{Api_File, FilesData};
use miohttp::{Request, Response, Code, Type};
use task_async::{self, Task};

pub struct Server {
    api_file: Api_File
}

impl Server {

    pub fn new(api_file: Api_File) -> Server {
        Server{
            api_file: api_file
        }
    }
    
    pub fn process(&self, request:Request) {
        
        if request.path().trim() == "/crash" {
            panic!("the simulated failure");
        }
        
        if request.path().trim() == "/post_action" {

            if request.is_post() {

                request.get_post(Box::new(move|request: Request, dane_opt: Option<Vec<u8>>|{

                    match dane_opt {

                        Some(dane) => {

                            let mes  = format!("odbieram dane postem: {}", dane.len());

                            let resp = Response::create(Code::Code200, Type::TextHtml, mes);
                            request.send(resp);
                        },

                        None => {

                            //nieobsłużenie spowoduje błąd 500
                        }
                    }
                }));

            } else {

                let mes  = format!("postownie: żądanie wysłane getem");

                let resp = Response::create(Code::Code200, Type::TextHtml, mes);
                request.send(resp);
            }

            return;
        }


        //serwujemy statyczną zawartość
        
        let path_src = "./static".to_owned() + request.path().trim();
        task_async::log_info(format!("Path requested: {}", &path_src));

        let path         = path_src.clone();
        let request_path = request.path().clone();

        let task = Task::new(Box::new(move|resonse : Option<(Response)>|{


            match resonse {
                Some(resp) => {
                    request.send(resp);
                },
                None => {
                    //domyślny mechanizm
                }
            };

        }));





        let task_get_file = task.async1(Box::new(move|task: Task<(Response)>, response: Option<FilesData>|{

            task_async::log_debug(format!("Invoked request's callback in response"));

            match response {

                Some(resp_result) => {
                    match resp_result {

                        Ok(buffer) => {

                            let buffer = buffer.to_owned();

                            let path         = Path::new(&path_src);
                            let content_type = Type::create_from_path(&path);

                            task_async::log_info(format!("200, {}, {}", content_type, request_path));

                            let response = Response::create_from_buf(Code::Code200, content_type, buffer);

                            task.result(response)
                        }

                        Err(err) => {

                            match err.kind() {

                                io::ErrorKind::NotFound => {

                                    let mess     = "Not found".to_owned();
                                    let response = Response::create(Code::Code404, Type::TextHtml, mess.clone());
                                    task_async::log_debug(format!("404, {}, {}. {:?} ", Type::TextHtml, request_path, err));

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

        
        let resp = Response::create(Code::Code200, Type::TextHtml, "Hello world2 -> ".to_owned() + request.path());
        
        request.send(resp);
    }
}

