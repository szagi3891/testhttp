use std::sync::Arc;


pub type StaticFunc<A,B> = Box<Fn(A) -> B + 'static + Send + Sync>;

pub trait Convert<A,C> {
    fn conv(&self, A) -> C;
}

pub enum Fnconvert<A,B,C> {
    First(Arc<StaticFunc<A,C>>),
    Next (Arc<StaticFunc<A,B>>, Box<Convert<B,C>>)
}

impl<A,B,C> Fnconvert<A,B,C> {
    
    fn new(funk: StaticFunc<A,C>) -> Fnconvert<A,B,C> {
        Fnconvert::First(Arc::new(funk))
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
            
            Fnconvert::Next(ref func, ref next_conv) => {
                
                next_conv.conv(func(value))
            }
        }
    }
}


/*
pub trait Convert<T,R> {
    fn conv (self : Box<Self>, T) -> R         where Self: Sized;
    fn clone(self : Box<Self>)    -> Box<Self> where Self: Sized;
}

//http://huonw.github.io/blog/2015/01/object-safety/

pub struct Fnconvert<T,R> {
    
    func : Arc<Box<Fn(T) -> R + 'static + Send + Sync>>,
}


impl<T, R> Fnconvert<T, R>
    where
        T : 'static + Send + Sync ,
        R : 'static + Send + Sync {
    
    pub fn new(func : Box<Fn(T) -> R + 'static + Send + Sync>) -> Box<Fnconvert<T, R>> {
        
        Box::new(Fnconvert {
            func : Arc::new(func)
        })
    }
}


impl<T,R> Convert<T,R> for Fnconvert<T,R> {
    
    fn conv(self: Box<Self>, value : T) -> R {
        
        match self.func {

            ref func => {

                func(value)
            }
        }
    }
    
    fn clone(self: Box<Self>) -> Box<Fnconvert<T,R>> {
        
        Box::new(Fnconvert {
            func : self.func.clone()
        })
    }
}
*/