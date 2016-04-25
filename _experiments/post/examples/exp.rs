use std::collections::HashMap;
use std::str;


pub type HashMapFn<T> = HashMap<String, Box<Fn(&mut T, Vec<u8>) -> bool>>;

pub trait ModelBind : Default {
    fn model_bind() -> HashMapFn<Self>;
}

pub trait ModelConvert<T> where Self: Sized {
    fn from(T) -> Result<Self, ()>;
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


macro_rules! struct_serialize {
    
    ($name:ident => $($element: ident: $ty: ty),+) => {
        
        #[derive(Default, Debug)]
        struct $name { $($element: $ty),+ }
        
        impl ModelBind for $name {

            fn model_bind() -> HashMapFn<$name> {

                let mut map: HashMapFn<$name> = HashMap::new();
                
                $(
                    map.insert(stringify!($element).to_owned(), Box::new(|model: &mut $name, value: Vec<u8>| -> bool {
                        match ModelConvert::from(value) {
                            Ok(value) => {
                                model.$element = value;
                                true
                            },
                            Err(()) => false,
                        }
                    }));
                    
                )+
                
                map
            }
        }
    }
}


struct_serialize!(
    Post =>
        wiek : String,
        rok : String,
        fff : String
);

fn main() { 
    
    println!("hello");
}

