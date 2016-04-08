use mio::{Token, Sender};
use asynchttp::miohttp::response::Response;


pub struct Respchan {
    //is_send : bool,
    token   : Token,
    sender  : Sender<(Token, Response)>,
}


impl Respchan {
    
    pub fn new(token: Token, sender: Sender<(Token, Response)>) -> Respchan {
        
        Respchan {
            //is_send : false,
            token   : token,
            sender  : sender
        }
    }
    
    pub fn send(self, response: Response) {
        
        //self.is_send = true;
        
        (self.sender).send((self.token, response)).unwrap();
    }
}

/*
&mut 
impl Drop for Respchan {

    fn drop(&mut self) {

        if self.is_send == false {
            
            panic!("unhandled request");
        }
    }
}
*/

