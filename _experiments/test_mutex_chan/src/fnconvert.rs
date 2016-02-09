use std::sync::Arc;
    
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