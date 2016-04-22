use std::collections::HashMap;
use std::str;

pub type HashMapFn<T> = HashMap<String, Box<Fn(&mut T, Vec<u8>) -> bool>>;

pub trait ModelBind {
    fn model_bind() -> (Self, HashMapFn<Self>);
    //fn bind() -> Option<Self>;
}

pub trait ModelConvert<T> where Self: Sized {
    fn from(T) -> Result<Self, ()>;
}

struct ModelBuilder<T> where T : ModelBind {
    model : T,
    map   : HashMapFn<T>,
}

impl<T> ModelBuilder<T> where T : ModelBind {
    
    fn new() -> ModelBuilder<T> {
        
        let (model, map) = T::model_bind();
        
        ModelBuilder {
            model : model,
            map   : map,
        }
    }
    
    fn set(&mut self, field: String, value: Vec<u8>) -> bool {
        
        match self.map.remove(&field) {
            Some(fn_set) => {
                fn_set(&mut self.model, value);
                true
            },
            None => {
                false
            },
        }
    }
    
    fn get(self) -> Option<T> {
        
        if self.map.len() == 0 {
            Some(self.model)
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
struct PostField {
    value : Vec<u8>,
}


impl ModelConvert<Vec<u8>> for PostField {
    
    fn from(value: Vec<u8>) -> Result<PostField, ()> {
        
        Ok(PostField {
            value : value
        })
    }
}

impl ModelConvert<Vec<u8>> for String {
    
    fn from(value: Vec<u8>) -> Result<String, ()> {
        
        match str::from_utf8(&value) {
            Ok(v) => {
                Ok(v.to_owned())
            },
            Err(_) => Err(()),
        }
    }
}


#[derive(Default, Debug)]
struct User {
    
    login : String,
    pass  : String,
    bad   : String,
}

impl ModelBind for User {
    
    fn model_bind() -> (User, HashMapFn<User>) {
        
        let model = Default::default();
        
        let mut map: HashMapFn<User> = HashMap::new();
        
        map.insert("login".to_owned(), Box::new(|model: &mut User, value: Vec<u8>| -> bool {
            match ModelConvert::from(value) {
                Ok(value) => {
                    model.login = value;
                    true
                },
                Err(()) => false,
            }
        }));
        
        map.insert("pass".to_owned() , Box::new(|model: &mut User, value: Vec<u8>| -> bool {
            
            match ModelConvert::from(value) {
                Ok(value) => {
                    model.pass = value;
                    true
                },
                Err(()) => false,
            }
        }));
        
        (model, map)
    }
}


fn make_model(login: Vec<u8>, pass: Vec<u8>, bad: Vec<u8>) -> Option<User> {
    
    let mut builder : ModelBuilder<User> = ModelBuilder::new();
    
    if builder.set("login".to_owned(), login) == false {
        return None
    }
    
    if builder.set("pass".to_owned(), pass) == false {
        return None;
    }
    
    if builder.set("bad".to_owned(), bad) == false {
        return None;
    }
    
    builder.get()
}

fn main() {
    
    let login : Vec<u8> = "Grzegorz".to_owned().into_bytes();
    let pass  : Vec<u8> = "tajne hasło".to_owned().into_bytes();
    let bad   : Vec<u8> = vec![129, 129, 129];
    
    let user_opt = make_model(login, pass, bad);
    
    /*
    match user_opt {
        Some(user) => {
            
            match login.String() && pass.String() {
                
            }
        }
    }
    */
    
    println!("Hello, world! {:?}", user_opt);
}




/*
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}
*/

/*
    w przypadku przenoszenia pól, np. do zapytania wstawiającego dane do bazy, zrobić iterator jakiś np.
*/


/*
http://blog.burntsushi.net/rust-error-handling/

trait From<T> {
    fn from(T) -> Self;
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

let string: String = From::from("foo");
let bytes: Vec<u8> = From::from("foo");

*/