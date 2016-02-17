use std::sync::Arc;

fn main() {
    println!("Hello, func world!");
}

pub type StaticFunc<A,B> = Fn(A) -> B + 'static + Send + Sync;

pub trait Convert<A,C> {
    fn conv(&self, A) -> C;
}

pub enum Fnconvert<A,B,C> {
    First(Arc<Box<StaticFunc<A,C>>>),
    Next (Arc<Box<StaticFunc<A,B>>>, Box<Convert<B,C>>)
}

impl<A,B,C> Convert<A,C> for Fnconvert<A,B,C> {
    
    fn conv(&self, value : A) -> C {
        
        match *self {
            
            Fnconvert::First(ref func) => {
                
                func(value)
            },
            
            Fnconvert::Next(ref func, ref next_conv) => {
                
                next_conv.conv(func(value))
            }
        }
    }
}

        /*
        match self {
            
            &Fnconvert::First(ref func) => {
                
                func(value)
            },
            
            &Fnconvert::Next(ref func, ref nextConv) => {
                
                nextConv.conv(func(value))
            }
        }
        */
        
