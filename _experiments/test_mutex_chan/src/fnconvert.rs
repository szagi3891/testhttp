
pub type StaticFunc<A,B> = Box<Fn(A) -> B + 'static + Send + Sync>;

pub trait Convert<A,C> {
    fn conv(&self, A) -> C;
}

pub enum Fnconvert<A,B,C>{
    
    First(StaticFunc<A,C>),
    Next (Box<Convert<A,B> + Send>, StaticFunc<B,C>)
}

impl<A,B,C> Fnconvert<A,B,C> {
    
    pub fn new(funk: StaticFunc<A,C>) -> Box<Fnconvert<A,B,C>> {
        Box::new(Fnconvert::First(funk))
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