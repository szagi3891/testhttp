macro_rules! struct_serialize {
    ($name:ident => $($element: ident: $ty: ty),*) => {
        struct $name { $($element: $ty),* }
    }
}

struct_serialize!(
    User =>
        x : u8,
        y : String,
        z : u32
);


fn main() { 
    
    println!("hello");
}
