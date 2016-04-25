macro_rules! struct_serialize {
    
    ($name:ident => $($element: ident: $ty: ty),*) => {
        
        struct $name { $($element: $ty),* }
        
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
                    
                ),*
                
                
                map
            }
        }
    }
}


struct_serialize!(
    Post =>
        //wiek : u32,
        rok : u32
);

fn main() { 
    
    println!("hello");
}


/*
struct_serialize!(
    User =>
        x : u8,
        y : String,
        z : u32,
        k : String
);
*/

