use std::io;
use std::path::Path;
use api_file::{ApiFile, FilesData};
use miohttp::{Request, Response, Code, Type};
use task_async::{self, Task};

pub struct Server {
    api_file: ApiFile
}

impl Server {

    pub fn new(api_file: ApiFile) -> Server {
        Server{
            api_file: api_file
        }
    }
    
    pub fn process(&self, request:Request) {
        
        self.test_crash(&request);
        
        if request.path().trim() == "/post_action" {

            self.test_post(request);
            return;
        }
        
        self.send_static(request);
        
    }
    
    fn test_crash(&self, request: &Request) {
        if request.path().trim() == "/crash" {
            panic!("the simulated failure");
        }
    }

    fn test_post(&self, request: Request) {

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
    }

    fn send_static(&self, request: Request) {
        let (path_disk, path_requested) = self.get_static_path(&request);
        
        task_async::log_info(format!("Path requested: {}", &path_disk));
        
        let task = Task::new(Box::new(move|resonse : Option<Response>|{

            match resonse {
                Some(resp) => {
                    request.send(resp);
                },
                None => {
                    //domyślny mechanizm
                }
            };

        }));


        let path_disk_clone = path_disk.clone();
        
        let task_get_file = task.async1(Box::new(move|task: Task<Response>, response: Option<FilesData>|{

            task_async::log_debug(format!("Invoked request's callback in response"));

            match response {

                Some(Ok(buffer)) => {

                    let buffer = buffer.to_owned();

                    let path         = Path::new(&path_disk);
                    let content_type = Type::create_from_path(&path);

                    task_async::log_info(format!("200, {}, {}", content_type, path_requested));

                    let response = Response::create_from_buf(Code::Code200, content_type, buffer);

                    task.result(response)
                }

                Some(Err(err)) => {

                    match err.kind() {

                        io::ErrorKind::NotFound => {

                            let mess     = "Not found".to_owned();
                            let response = Response::create(Code::Code404, Type::TextHtml, mess.clone());
                            task_async::log_debug(format!("404, {}, {}. {:?} ", Type::TextHtml, path_requested, err));

                            task.result(response)
                        }
                        _ => {

                            task_async::log_error(format!("errrrr {:?}", err));
                        }
                    }

                }

                None => {
                    //api nie odpowiedziało
                }
            }
        }));
        
        self.api_file.get_file(path_disk_clone, task_get_file);


        /*
        let resp = Response::create(Code::Code200, Type::TextHtml, "Hello world2 -> ".to_owned() + request.path());        
        request.send(resp);
        */
    }
    fn get_static_path(&self, request: &Request) -> (String, String) {

        let path = request.path().trim();
        
        let path_requested = if path == "/" {
            "/index.html"
        } else {
            path
        };

        let path_disk = "./static".to_owned() + path_requested;

        (path_disk, path_requested.to_owned())
    }
}

