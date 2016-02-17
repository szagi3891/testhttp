fn main() {
    
    println!("Hello, func world!");
    
    let tt: Box<Fnconvert<u32, u32, u32>> = Box::new(Fnconvert::new(create_iden::<u32>()));
    
    let tt2 = Fnconvert::Next(tt, Box::new(|argin: u32| -> u64 {
        argin as u64 + 9
    }));
    
    println!("aaa {}", tt2.conv(10));
}

fn create_iden<A>() -> Box<Fn(A) -> A + Send + Sync + 'static> {
    Box::new(|argin: A| -> A {
        argin
    })
}

pub type StaticFunc<A,B> = Box<Fn(A) -> B + 'static + Send + Sync>;

pub trait Convert<A,C> {
    fn conv(&self, A) -> C;
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
    
}



/*
src/main.rs:21:28: 21:46 error: the trait `Convert` cannot be made into an object [E0038]
src/main.rs:21     Next (StaticFunc<A,B>, Box<Convert<B,C>>)
                                          ^~~~~~~~~~~~~~~~~~
src/main.rs:21:28: 21:46 help: run `rustc --explain E0038` to see a detailed explanation
src/main.rs:21:28: 21:46 note: method `transform` has generic type parameters

pub trait Convert<A,C> {
    fn conv(&self, A) -> C;
    //fn transform<D>(self, StaticFunc<C,D>) -> Fnconvert<A,C,D>;
}

    fn transform<D>(self, func_transform: StaticFunc<C,D>) -> Fnconvert<A,C,D> {
        
        Fnconvert::Next(self, func_transform)
    }
*/



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
        
