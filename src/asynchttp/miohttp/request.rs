use std::collections::HashMap;
use mio;
use httparse;
use std::io::{Error, ErrorKind};

use asynchttp::miohttp::response;
use asynchttp::miohttp::httpstr;

pub struct PreRequest {
    method : httpstr::MethodName,
    path : httpstr::Path,
    version : u8,
    headers : HashMap<httpstr::HeaderName, httpstr::HeaderValue>,
}

impl PreRequest {

    pub fn new(req: httparse::Request) -> Result<PreRequest, Error> {

        match (req.method, req.path, req.version) {

            (Some(method), Some(path), Some(version)) => {

                let mut pre_request = PreRequest{
                    method  : [0; httpstr::METHOD_NAME_LENGTH],
                    path    : [0; httpstr::PATH_LENGTH],
                    version : version,
                    headers : HashMap::new(),
                };

                httpstr::copy(&method.as_bytes(), &mut pre_request.method);
                httpstr::copy(&path.as_bytes(), &mut pre_request.path);

                for header in req.headers {
                    let mut key = [0u8; httpstr::HEADER_NAME_LENGTH];
                    let mut value = [0u8; httpstr::HEADER_VALUE_LENGTH];
                    httpstr::copy(&header.name.as_bytes(), &mut key);
                    httpstr::copy(&header.value, &mut value);

                    pre_request.headers.insert(key, value);
                }

                Ok(pre_request)
            }
            _ => {

                //TODO - komunikat ma bardziej szczegółowo wskazywać gdzie wystąpił błąd
                Err(Error::new(ErrorKind::InvalidInput, "Błąd tworzenia odpowiedzi"))
            }
        }
    }
    
    
    pub fn bind(self, token: &mio::Token, resp_chanel: mio::Sender<(mio::Token, response::Response)>) -> Request {
        Request {
            is_send     : false,
            method      : self.method,
            path        : self.path,
            version     : self.version,
            headers     : self.headers,
            token       : token.clone(),
            resp_chanel : resp_chanel
        }
    }
}



pub struct Request {
    is_send     : bool,
    pub method  : httpstr::MethodName,
    pub path    : httpstr::Path,
    pub version : u8,
    headers     : HashMap<httpstr::HeaderName, httpstr::HeaderValue>,
    token       : mio::Token,
    resp_chanel : mio::Sender<(mio::Token, response::Response)>,
}

/*
http://hyper.rs/hyper/hyper/header/struct.Headers.html
                ta biblioteka wykorzystuje nagłówki dostarczane przez hyper-a
https://github.com/tailhook/rotor-http/blob/master/src/http1.rs
*/


impl Request {

    pub fn is_header_set(&self, name: &str, value: &str) -> bool {
        
        match self.headers.get(name.as_bytes()) {
            
            Some(get_value) => {
                httpstr::eq(get_value, value.trim().as_bytes())
            }
            
            None => false
        }
    }
    
    pub fn send(mut self, response: response::Response) {
        
        let _ = self.resp_chanel.send((self.token, response));
        
        self.is_send = true;
    }
}

impl Drop for Request {

    fn drop(&mut self) {
        
        if self.is_send == false {
            
            let _ = self.resp_chanel.send((self.token, response::Response::create_500()));
        }
    }

}

