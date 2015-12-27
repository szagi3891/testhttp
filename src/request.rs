use httparse;
use std::collections::HashMap;
use std;


#[derive(Debug)]
pub struct Request {
    method : String,
    path : String,
    version : u8,
    headers : HashMap<String, String>,
}

impl Request {
    
    pub fn new(req: httparse::Request) -> Result<Request, String> {
        
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
                    
                    match headers.insert(key.clone(), value) {
                        None => {}      //insert ok
                        Some(_) => {
                            return Err(format!("double header: {}", &key));
                        }
                    };
                }
                
                Ok(Request{
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
        
        /*
        println!("httparse::Request:");
        println!("method : {:?}", req.method);
        println!("path : {:?}", req.path);
        println!("version : {:?}", req.version);
        //println!("headers : {:?}", req.headers);

        for header in req.headers {
            let str_header = String::from_utf8_lossy(header.value);
            println!("  {} : {}", header.name, str_header);
        }
        
        panic!("STOP");
        */
    }
}

