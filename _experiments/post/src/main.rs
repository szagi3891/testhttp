use std::collections::HashMap;

pub type HashMapFn<T> = HashMap<String, Box<Fn(&mut T, Vec<u8>)>>;

pub trait ModelBind {
    fn model_bind() -> (Self, HashMapFn<Self>);
}


#[derive(Default)]
struct User {
    
    login : Vec<u8>,
    pass  : Vec<u8>,
}

impl ModelBind for User {
    
    fn model_bind() -> (User, HashMapFn<User>) {
        
        let model = Default::default();
        
        let mut map: HashMapFn<User> = HashMap::new();
        
        map.insert("login".to_owned(), Box::new(|model: &mut User, value: Vec<u8>|{
            model.login = From::from(value);
        }));
        
        map.insert("pass".to_owned() , Box::new(|model: &mut User, value: Vec<u8>|{
            model.pass = From::from(value);
        }));
        
        (model, map)
    }
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
        
        //TODO
        false
    }
    
    fn get() -> Option<T> {
        
        //TODO
        None
    }
}

fn make_model(login: Vec<u8>, pass: Vec<u8>) -> Option<User> {
    
    
    let mut builder : ModelBuilder<User> = ModelBuilder::new();
    
    if builder.set("login".to_owned(), login) == false {
        return None
    }
    
    if builder.set("pass".to_owned(), pass) == false {
        return None;
    }
    
    builder.get()
    
    None
}

fn main() {
    
    let login : Vec<u8> = "Grzegorz".to_owned().into_bytes();
    let pass  : Vec<u8> = "tajne hasło".to_owned().into_bytes();
    
    let user_opt = make_model(login, pass);
    
    
    
    println!("Hello, world!");
}




/*
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}*/

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