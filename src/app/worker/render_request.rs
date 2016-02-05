use std::io;
use std::path::Path;
use comm::mpmc::bounded::Channel;
use inlinable_string::{InlinableString, StringExt};

use asynchttp::log;
use asynchttp::miohttp::{request, response};
use app::api;

pub fn render_request(request: request::Request, tx_api_request: &Channel<api::Request>) {
    
    
    let mut path_src = InlinableString::new();
    path_src.push_str("./static");
    path_src.push_str(request.path.trim());
    log::info(format!("Path requested: {}", path_src.as_ref()));
    
    
    let path = path_src.clone();
    
    tx_api_request.send_sync(api::Request::GetFile(path_src, Box::new(move|data: api::FilesData|{

        log::debug(format!("Invoked request's callback in response"));

        match data {

            Ok(buffer) => {

                let path         = Path::new(path.as_ref());
                let content_type = response::Type::create_from_path(&path);

                log::info(format!("200, {}, {}", content_type, request.path.as_ref()));

                let response = response::Response::create_from_buf(response::Code::Code200, content_type, buffer);

                request.send(response);
            }

            Err(err) => {

                match err.kind() {

                    io::ErrorKind::NotFound => {

                        let response = response::Response::create(response::Code::Code404, response::Type::TextHtml, String::from("Not found").into_bytes());
                        log::debug(format!("404, {}, {}. {:?} ", response::Type::TextHtml, request.path.as_ref(), err));
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
