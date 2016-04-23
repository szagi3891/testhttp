use std::collections::HashMap;
use std::str;

extern crate rustc_serialize;
//use rustc_serialize::json;

pub type HashMapFn<T> = HashMap<String, Box<Fn(&mut T, Vec<u8>) -> bool>>;

pub trait ModelBind : Default {
    fn model_bind() -> HashMapFn<Self>;
}

pub trait ModelConvert<T> where Self: Sized {
    fn from(T) -> Result<Self, ()>;
}

struct ModelBuilder<T> where T : ModelBind {
    model : T,
    map   : HashMapFn<T>,
}


#[derive(PartialEq)]
enum ModelBuilderStatus {
    ErrModel,
    ErrConvert,
    Ok
}

impl<T> ModelBuilder<T> where T : ModelBind {
    
    fn new() -> ModelBuilder<T> {
        
        ModelBuilder {
            model : Default::default(),
            map   : T::model_bind(),
        }
    }
    
    fn set(&mut self, field: String, value: Vec<u8>) -> ModelBuilderStatus {
        
        match self.map.remove(&field) {
            Some(fn_set) => {
                if fn_set(&mut self.model, value) {
                    ModelBuilderStatus::Ok
                } else {
                    ModelBuilderStatus::ErrConvert
                }
            },
            None => {
                ModelBuilderStatus::ErrModel
            },
        }
    }
    
    fn get(self) -> Option<T> {
        
        if self.map.len() == 0 {
            Some(self.model)
        } else {
            None                            //nie udało się dopasować wszystkich pól
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
    
    fn model_bind() -> HashMapFn<User> {
        
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
        
        map.insert("bad".to_owned() , Box::new(|model: &mut User, value: Vec<u8>| -> bool {
            
            match ModelConvert::from(value) {
                Ok(value) => {
                    model.bad = value;
                    true
                },
                Err(()) => false,
            }
        }));
        
        map
    }
    
    //fn model_get -> zwraca hashmapę ? a może sprytny iterator jakiś
    /*
        dobrze jeśli ta funkcja byłaby parametryzowana generycznie
        automatycznie wartości zwracane byłyby konwertowane na typ parametur generyczny
        
        dzięki temu można by zrobić łatwiejsze rzutowanie na stringa np. bazodanowego ...
            który byłby osobnym polem
    */
    
    /*
        Fn (callback_result: Option<model>) {

            model::Default()
            
            let mut mapa i jakiś stan
            getPost(
                Box<Fn(name: String, value: Vec<u8>) -> bool>,          //true, wartość, ok, kontynuuj
                Box<FnOnce()>                                           //gdy już nie będzie kolejnych danych
            )
        }
    */
}


fn make_model(login: Vec<u8>, pass: Vec<u8>, bad: Vec<u8>) -> Option<User> {
    
    let mut builder : ModelBuilder<User> = ModelBuilder::new();
    
    if builder.set("login".to_owned(), login) != ModelBuilderStatus::Ok {
        return None
    }
    
    if builder.set("pass".to_owned(), pass) != ModelBuilderStatus::Ok {
        return None;
    }
    
    if builder.set("bad".to_owned(), bad) != ModelBuilderStatus::Ok {
        return None;
    }
    
    builder.get()
}

//TODO - zrobić testy, sprawdzające różne warianty


#[derive(RustcEncodable, RustcDecodable)]
struct test {
    name : String,
    password : String,
}

//rustc ./src/main.rs -Z unstable-options --pretty expanded




fn main() {
    
    let login  = "Grzegorz".to_owned().into_bytes();
    let pass   = "tajne hasło".to_owned().into_bytes();
    let bad    = vec![129, 200, 200];
    //let bad    = vec![];
    //let bad = "taj".to_owned().into_bytes();
    
    let user_opt = make_model(login, pass, bad);
    
    println!("Hello, world2! {:?}", user_opt);
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