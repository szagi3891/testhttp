use asynchttp::log;
use asynchttp::miohttp::{request, response};
use chan::Sender;
use std::path::Path;
use std::io;
use app::api;


pub fn render_request(request: request::Request, tx_files_path: &Sender<(String, api::CallbackFD)>) {
    
    
    let path_src = "./static".to_owned() + request.path.trim();
    log::info(format!("Path requested: {}", &path_src));
    
    
    
    //tutaj
    
    //kanał może przyjmować tylko "taski" typ
    //do stworzenia nowego tasku, potrzebny jest identyfikator tasku
    //request.task() - pobieramy nowy obiekt, zwiększa się licznik referencji z taskami
    //obiekt requestu do api
        //taskLicznik, enumRequestu
    
    /*
    
    api::Request(task_count.clone(), api::Request::GetFile(path_src.clone(), Callback(move|data: api::FilesData|{
    });
    */
    
    //tx_files_path.send((path_src.clone(), async::new(request.task(), |data: FilesData|{
    tx_files_path.send((path_src.clone(), Box::new(move|data: api::FilesData|{

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