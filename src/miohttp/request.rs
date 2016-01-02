use httparse;
use std::collections::HashMap;
use std;
use mio;
use miohttp::response;


#[derive(Debug)]
pub struct PreRequest {
    method : String,
    path : String,
    version : u8,
    headers : HashMap<Box<String>, String>,
}

impl PreRequest {

    pub fn new(req: httparse::Request) -> Result<PreRequest, String> {

        match (req.method, req.path, req.version) {

            (Some(method), Some(path), Some(version)) => {

                let mut headers = HashMap::new();

                for header in req.headers {

                    let key   = header.name.to_string();

                    let value = match std::str::from_utf8(header.value) {
                        Ok(value) => value.to_string(),
                        Err(err) => {
                            return Err(format!("header {}, error utf8 sequence: {}", key, err))
                        }
                    };

                    match headers.insert(Box::new(key.clone()), value) {
                        None => {}      //insert ok
                        Some(_) => {
                            return Err(format!("double header: {}", &key));
                        }
                    };
                }

                Ok(PreRequest{
                    method  : method.to_string(),
                    path    : path.to_string(),
                    version : version,
                    headers : headers,
                })
            }
            _ => {

                //TODO - komunikat ma bardziej szczegółowo wskazywać gdzie wystąpił błąd
                Err("Błąd tworzenia odpowiedzi".to_string())
            }
        }
    }
    
    
    pub fn bind(self, token: &mio::Token, resp_chanel: mio::Sender<(mio::Token, response::Response)>) -> Request {
        Request {
            method      : self.method,
            path        : self.path,
            version     : self.version,
            headers     : self.headers,
            token       : token.clone(),
            resp_chanel : resp_chanel
        }
    }
}



#[derive(Debug)]
pub struct Request {
    pub method  : String,
    pub path    : String,
    pub version : u8,
    headers     : HashMap<Box<String>, String>,
    token       : mio::Token,
    resp_chanel : mio::Sender<(mio::Token, response::Response)>,
}

/*
http://hyper.rs/hyper/hyper/header/struct.Headers.html
                ta biblioteka wykorzystuje nagłówki dostarczane przez hyper-a
https://github.com/tailhook/rotor-http/blob/master/src/http1.rs
*/


//TODO - trzeba będzie mu zaimplementować dropa


impl Request {

    pub fn is_header_set(&self, name: &str, value: &str) -> bool {
        
        match self.headers.get(&Box::new(name.to_string())) {
            
            Some(get_value) => {
                get_value == value.trim()
            }
            
            None => false
        }
    }
    
    pub fn send(self, response: response::Response) {
        
        let _ = self.resp_chanel.send((self.token, response));
    }
}


