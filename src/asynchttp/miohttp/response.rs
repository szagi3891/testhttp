use std::fmt;
use std::path::Path;

pub enum Code {
    Code200,
    Code400,
    Code404,
    Code500,
}

//https://en.wikipedia.org/wiki/List_of_HTTP_status_codes

impl Code {
    fn to_str(&self) -> &str {
        match *self {
            Code::Code200 => "200 OK",
            Code::Code400 => "400 Bad Request",
            Code::Code404 => "404 Not Found",
            Code::Code500 => "500 Internal Server Error",
        }
    }
}

pub enum Type {
    TextHtml,
    TextPlain,
    ImageJpeg,
    ImagePng,
}

impl Type {
    fn to_str(&self) -> &str {
        match *self {
            Type::TextHtml => "text/html; charset=utf-8",
            Type::TextPlain => "text/plain",
            Type::ImageJpeg => "image/jpeg",
            Type::ImagePng => "image/png",
        }
    }


    pub fn create_from_path(path: &Path) -> Type {
        
        // TODO: Match on strings is slow, maybe some b-tree?
        
        match path.extension() {
            
            Some(ext) => match ext.to_str() {
                Some("txt")  => Type::TextPlain,
                Some("jpg")  => Type::ImageJpeg,
                Some("png")  => Type::ImagePng,
                //Some("html") => Type::TextHtml,
                Some(_)      => Type::TextHtml,
                None         => Type::TextHtml,
            },
            
            None => Type::TextHtml,
        }
    }

}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug)]
pub struct Response {
    message : Vec<u8>,
}


impl Response {
    
    pub fn as_bytes(self) -> Vec<u8> {
        self.message
    }
    
    fn append_string(&mut self, line: String) {
        self.message.append(&mut (line + "\r\n").into_bytes());
    }
    
    fn create_headers(code: Code, typ: Type, length: usize) -> Response {
        
        let mut response = Response {
            message : Vec::new()
        };
        
        response.append_string("HTTP/1.1 ".to_owned() + code.to_str());
        response.append_string("Date: Thu, 20 Dec 2001 12:04:30 GMT".to_owned());
        response.append_string("Content-Type: ".to_owned() + typ.to_str());
        response.append_string("Connection: keep-alive".to_owned());
        response.append_string(format!("Content-length: {}", length).to_owned());
        response.append_string("".to_owned());
        
        response
    }
    
    pub fn create(code: Code, typ: Type, body: String) -> Response {
        
        let mut response = Response::create_headers(code, typ, body.len());
        
        response.append_string(body);
        
        response
    }
    
    pub fn create_from_buf(code: Code, typ: Type, mut body: Vec<u8>) -> Response {
        
        let mut response = Response::create_headers(code, typ, body.len());
        
        response.message.append(&mut body);
        
        response
    }
    
    pub fn create_500() -> Response {
        Response::create(Code::Code500, Type::TextHtml, "500 Internal Server Error".to_owned())
    }
    
    pub fn create_400() -> Response {
        Response::create(Code::Code400, Type::TextHtml, "400 Bad Request".to_owned())
    }
    
    /*
    let mut out: Vec<u8> = Vec::new();
    out.append(&mut ("HTTP/1.1 ".to_owned() + code.to_str() + "\r\n").into_bytes());
    out.append(&mut ("Date: Thu, 20 Dec 2001 12:04:30 GMT".to_owned() + "\r\n").into_bytes());

    out.append(&mut ("Content-Type: ".to_owned() + typ.to_str() + "\r\n").into_bytes());
    out.append(&mut ("Connection: keep-alive".to_owned() + "\r\n").into_bytes());
    out.append(&mut (format!("Content-length: {}", body.len()).to_owned() + "\r\n").into_bytes());
    out.append(&mut ("\r\n".to_owned().into_bytes()));
    out.append(&mut (body.into_bytes()));
    */
    /*
    Response {
        message : out
    }*/

    /*
    let mut out: Vec<String> = Vec::new();
    out.push("HTTP/1.1 ".to_owned() + code.to_str());
    out.push("Date: Thu, 20 Dec 2001 12:04:30 GMT".to_owned());      //TODO - trzeba wyznaczać aktualną wartość daty
    out.push("Content-Type: ".to_owned() + typ.to_str());
    out.push("Connection: keep-alive".to_owned());
    out.push(format!("Content-length: {}", body.len()).to_owned());
    out.push("".to_owned());
    out.push(body);

    let message = out.join("\r\n");

    let mut resp_vec: Vec<u8> = Vec::new();

    for byte in message.as_bytes() {
        resp_vec.push(byte.clone());
    }
    */

    /*
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);
    */

    /*
        konwertuje string na tablicę bajtów

    let s = String::from("hello");
    let bytes = s.into_bytes();
    assert_eq!(bytes, [104, 101, 108, 108, 111]);
    */ 
    
    /*
    println!("dd {}", ["hello", "world"].join(" "));
    println!("tt {}", ["asda 11".to_owned(), "asda 22".to_owned()].join(" "));
    
    let hello = "Hello ".to_owned();
    let world = "world!";
    let hello_world = hello + world;

    let hello = "Hello ".to_owned();
    let world = "world!".to_owned();
    let hello_world = hello + &world;
    
    "sfsfsd".to_owned() == "das"
    */
}

