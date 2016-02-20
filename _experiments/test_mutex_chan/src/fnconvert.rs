use std::sync::Arc;
    
pub type StaticFunc<A,B> = Box<Fn(A) -> B + Send + Sync + 'static>;

pub trait Convert<A,C> {
    fn conv(&self, A) -> C;
    fn clone(&self) -> Box<Convert<A,C> + Send>;
}

pub enum Fnconvert<A,B,C>
    where
        A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
    
    First(Arc<StaticFunc<A,C>>),
    Next (Box<Convert<A,B> + Send>, Arc<StaticFunc<B,C>>)
}

impl<A,B,C> Fnconvert<A,B,C>
    where
        A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
    
    pub fn new(funk: StaticFunc<A,C>) -> Box<Fnconvert<A,B,C>> {
        Box::new(Fnconvert::First(Arc::new(funk)))
    }
}

impl<A,B,C> Convert<A,C> for Fnconvert<A,B,C>
where
        A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
    
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
    
    fn clone(&self) -> Box<Convert<A,C> + Send> {
        
        match *self {
            
            Fnconvert::First(ref func) => {
                
                let new_copy: Fnconvert<A,B,C> = Fnconvert::First(func.clone());
                
                Box::new(new_copy)
            },
            
            Fnconvert::Next(ref next_conv, ref func) => {
                
                let new_copy: Fnconvert<A,B,C> = Fnconvert::Next((*next_conv).clone(), func.clone());
                
                Box::new(new_copy)
            }
        }
    }
}