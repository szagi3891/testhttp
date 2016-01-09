use miohttp::{request, response, log};
use chan::Sender;
use std::boxed::FnBox;
use std::path::Path;
use std::io;


pub fn render_request(request: request::Request, tx_files_path: Sender<(String, Box<FnBox(Result<Vec<u8>, io::Error>) + Send + 'static + Sync>)>) {
    
    let path_src = "./static".to_owned() + request.path.trim();

    log::info(format!("Path requested: {}", &path_src));

    tx_files_path.send((path_src.clone(), Box::new(move|data: Result<Vec<u8>, io::Error>|{

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

                        let mess     = "Not fund".to_string();
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