use std::collections::HashMap;


struct user {
    
    login : String,
    pass  : String,
}

impl user {
    
    fn model_builder() -> (user, HashMap<String, user>) {
        
        let model = user {
            login : "".to_owned(),
            pass  : "".to_owned(),
        };
        
        let map = HashMap::new();
        
        (model, map)
    }
}


struct model_builder<T> {
    model : T,
    map   : HashMap<String, T>,
}

impl<T> model_builder<T> {
    
    fn new(inject : (T, HashMap<String, T>)) -> model_builder<T> {
        
        let (model, map) = inject;
        
        model_builder {
            model : model,
            map   : map,
        }
    }
    
    fn set(&mut self, value: Vec<u8>) -> bool {
        
        false
    }
}


fn main() {
    
    let login    : Vec<u8> = "Grzegorz".to_owned().into_bytes();
    let password : Vec<u8> = "tajne hasło".to_owned().into_bytes();
    
    let builder = model_builder::new(user::model_builder());
    
    /*
    
    let konwerter = konwerter::new::<user>();
    
    let result =  konwerter.set("login", login);
    
    if result == false {
        return None
    }
    
    let result2 = konwerter.set("password", password);
    
    if result2 == false {
        return None;
    }
    
    konwerter.get()         //zwraca opcję
    
    */
    
    println!("Hello, world!");
}
