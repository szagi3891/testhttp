use std::io::{Error, ErrorKind};
use std::ops::Deref;
use std::collections::HashMap;
use mio;
use httparse;
use inlinable_string::{InlinableString, StringExt};

use asynchttp::miohttp::response;

pub struct PreRequest {
    method : InlinableString,
    path : InlinableString,
    version : u8,
    headers : HashMap<InlinableString, InlinableString>,
}

impl PreRequest {

    pub fn new(req: httparse::Request) -> Result<PreRequest, Error> {

        match (req.method, req.path, req.version) {

            (Some(method), Some(path), Some(version)) => {

                let mut pre_request = PreRequest{
                    method  : InlinableString::from(method),
                    path    : InlinableString::from(path),
                    version : version,
                    headers : HashMap::new(),
                };

                for header in req.headers {
                    pre_request.headers.insert(
                        InlinableString::from(header.name),
                        InlinableString::from(InlinableString::from_utf8_lossy(header.value).deref())
                    );
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
    pub method  : InlinableString,
    pub path    : InlinableString,
    pub version : u8,
    headers     : HashMap<InlinableString, InlinableString>,
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
        
        match self.headers.get(name) {
            
            Some(get_value) => {
                get_value == value.trim()
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

