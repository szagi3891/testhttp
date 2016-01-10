use asynchttp::log;
use asynchttp::miohttp::{request, response};
use chan::Sender;
use std::path::Path;
use std::io;
use app::api;


pub fn render_request(request: request::Request, tx_api_request: &Sender<api::Request>) {
    
    
    let path_src = "./static".to_owned() + request.path.trim();
    log::info(format!("Path requested: {}", &path_src));
    
    
    
    let path = path_src.clone();
    
    tx_api_request.send(api::Request::GetFile(path, Box::new(move|data: api::FilesData|{

        match data {

            Ok(buffer) => {

                let buffer = buffer.to_owned();

                let path         = Path::new(&path_src);
                let content_type = response::Type::create_from_path(&path);

                log::info(format!("200, {}, {}", content_type, request.path));

                let response = response::Response::create_from_buf(response::Code::Code200, content_type, buffer);

                request.send(response);
            }

            Err(err) => {

                match err.kind() {

                    io::ErrorKind::NotFound => {

                        let mess     = "Not fund".to_owned();
                        let response = response::Response::create(response::Code::Code404, response::Type::TextHtml, mess);
                        request.send(response);
                    }
                    _ => {

                        log::error(format!("errrrr {:?}", err));
                    }
                }

            }
        }
    })));
}