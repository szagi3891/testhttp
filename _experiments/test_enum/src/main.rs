fn main() {
    
    println!("Hello, func world!");
    
    let tt: Box<Fnconvert<u32, u32, u32>> = Box::new(Fnconvert::new(Box::new(|argin: u32| -> u32 {
        argin + 11
    })));
    
    let tt2 = Fnconvert::Next(tt, Box::new(|argin: u32| -> u64 {
        argin as u64 + 9
    }));
    
    println!("aaa {}", tt2.conv(10));
}

pub type StaticFunc<A,B> = Box<Fn(A) -> B + 'static + Send + Sync>;

pub trait Convert<A,C> {
    fn conv(&self, A) -> C;
    //fn transform<D>(self, StaticFunc<C,D>) -> Fnconvert<A,C,D>;
}

pub enum Fnconvert<A,B,C> {
    First(StaticFunc<A,C>),
    Next (Box<Convert<A,B>>, StaticFunc<B,C>)
}

impl<A,B,C> Fnconvert<A,B,C> {
    
    fn new(funk: StaticFunc<A,C>) -> Fnconvert<A,B,C> {
        Fnconvert::First(funk)
    }
}

impl<A,B,C> Convert<A,C> for Fnconvert<A,B,C> {
    
    fn conv(&self, value : A) -> C {
        
        match *self {
            
            Fnconvert::First(ref func) => {
                
                func(value)
            },
            
            Fnconvert::Next(ref next_conv, ref func) => {
                
                func(next_conv.conv(value))
            }
        }
    }
    
    /*
    fn transform<D>(self, func_transform: StaticFunc<C,D>) -> Fnconvert<A,C,D> {
        
        Fnconvert::Next(self, func_transform)
    }*/
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
        
