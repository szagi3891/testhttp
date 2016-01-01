#[derive(Debug)]
pub struct Response {
    pub message : String,
}

pub enum Type {
    html
}

impl Response {

    //from_text(numer, str)

    //np. from_text(400, "błąd parsowania")

    pub fn from_string(mess: String) -> Response {
        Response {
            message : mess
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.message.as_bytes()
    }
    
    pub fn create(code: u16, typ: Type, body: String) -> Response {
        
        //create(200, response::Type::html, response_body)));
        
        let message = format!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\nConnection: keep-alive\r\nContent-length: {}\r\n\r\n{}", body.len(), body);
        
        Response {
            message : message
        }
    }
    

    //TODO - test response
    /*
    let response_body = format!("Hello user: {} - {}", time_current.sec, time_current.nsec);
    let response = format!("HTTP/1.1 200 OK\r\nDate: Thu, 20 Dec 2001 12:04:30 GMT \r\nContent-Type: text/html; charset=utf-8\r\nConnection: keep-alive\r\nContent-length: {}\r\n\r\n{}", response_body.len(), response_body);
    
    //let _ = resp_chanel.send((token, response::Response::from_string(response)));
    */
    
    //println!("przesłano kanał z odpowiedzią : {:?}", req);
}

