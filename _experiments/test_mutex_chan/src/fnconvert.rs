use std::sync::Arc;


pub trait Convert<T,R> {
    fn conv(self : Box<Self>, T) -> R;
    fn clone(self : Box<Self>) -> Box<Self>;
}

//http://huonw.github.io/blog/2015/01/object-safety/

pub struct Fnconvert<T,R> {
    
    func : Arc<Box<Fn(T) -> R + 'static + Send + Sync>>,
}


impl<T, R> Fnconvert<T, R>
    where
        T : 'static + Send + Sync + Clone ,
        R : 'static + Send + Sync + Clone {
    
    pub fn new(func : Box<Fn(T) -> R + 'static + Send + Sync>) -> Fnconvert<T, R> {
        
        Fnconvert {
            func : Arc::new(func)
        }
    }
    
    pub fn conv(&self, value : Box<T>) -> R {
        
        match self.func {

            ref func => {

                func((*value).clone())
            }
        }
    }
            
    pub fn clone(&self) -> Fnconvert<T,R> {
        
        Fnconvert {
            func : self.func.clone()
        }
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

/*
next <T,R,K>
    Arc<Box<Fn(T) -> R + 'static + Send + Sync>>,
    Convert<R, K>
*/
