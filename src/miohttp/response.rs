pub enum Code {
    Code200,
    Code404,
}

//https://en.wikipedia.org/wiki/List_of_HTTP_status_codes

impl Code {
    fn to_str(&self) -> &str {
        match *self {
            Code::Code200 => "200 OK",
            Code::Code404 => "404 Not Found",
        }
    }
}

pub enum Type {
    Html
}

impl Type {
    fn to_str(&self) -> &str {
        match *self {
            Type::Html => "text/html; charset=utf-8",
        }
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
    
    pub fn create(code: Code, typ: Type, body: String) -> Response {
        
        let mut out: Vec<String> = Vec::new();
        out.push("HTTP/1.1 ".to_string() + code.to_str());
        out.push("Date: Thu, 20 Dec 2001 12:04:30 GMT".to_string());      //TODO - trzeba wyznaczać aktualną wartość daty
        out.push("Content-Type: ".to_string() + typ.to_str());
        out.push("Connection: keep-alive".to_string());
        out.push(format!("Content-length: {}", body.len()).to_string());
        out.push("".to_string());
        out.push(body);
        
        let message = out.join("\r\n");
        
        //TODO - występuje kopiowanie pamięci, znaleźć lepszy sposób na konwersję tych danych
        
        let mut resp_vec: Vec<u8> = Vec::new();
        
        for byte in message.as_bytes() {
            resp_vec.push(byte.clone());
        }
        
        
        Response {
            message : resp_vec
        }
    }
    
    /*
    println!("dd {}", ["hello", "world"].join(" "));
    println!("tt {}", ["asda 11".to_string(), "asda 22".to_string()].join(" "));
    
    let hello = "Hello ".to_string();
    let world = "world!";
    let hello_world = hello + world;

    let hello = "Hello ".to_string();
    let world = "world!".to_string();
    let hello_world = hello + &world;
    
    "sfsfsd".to_string() == "das"
    */
}

