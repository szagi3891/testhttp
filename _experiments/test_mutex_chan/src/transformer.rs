use std::sync::{Arc, Mutex};
use query::Query;
use outvalue::Outvalue;
use transport::{Transport, TransportOut};

/*
pub trait TransformerInterface<R> {
    fn transform<K>(self : Box<Self>, Box<Fn(R) -> K>) -> (TransportOut<K>, TransformerInterface<K>);
}
*/


pub struct Transformer<T, R> {
    
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Outvalue<R>>,
    pub transform : Box<Fn(T) -> R + Send>,
}


impl<T:Send+Clone+'static, R:Send+Clone+'static> Transformer<T, R> {
    
    fn transform<K>(self: Box<Self>, new_outvalue: &Arc<Outvalue<K>>, new_transform: &Box<Fn(R) -> K + Send>) -> (Transport<T,K>, Transformer<T,K>) {
        
        let transport = Transport {
            query    : self.query.clone(),
            outvalue : new_outvalue.clone(),
            transform : glue::<T,R,K>(self.transform, new_transform),
        };
        
        let transformer = Transformer {
            query     : self.query.clone(),
            outvalue  : new_outvalue.clone(),
            transform : glue::<T,R,K>(self.transform, new_transform),
        };
        
        
        (transport, transformer)
    }
}


fn glue<T,R,K>(fn1: Box<Fn(T) -> R + Send>, fn2: &Box<Fn(R) -> K + Send>) -> Box<Fn(T) -> K + Send> {
    
    let fun1 = fn1.clone();
    let fun2 = fn2.clone();
    
    Box::new(move|arg: T| -> K {
        
        fun2(fun1(arg))
    })
}

/*

impl<T:Send+Clone+'static, R:Send+Clone+'static> TransformerInterface<T> for Transformer<T, R> {
    
    fn transform(self: Box<Self>, new_transform: Box<Fn(R) -> K>) -> (Transport<T,K>, Transformer<T,K>) {
        
        
        
        let transport = Transport {
            query    : self.query.clone(),
            outvalue : self.outvalue.clone(),
            transform : glue(self.transform, new_transform),
        };
        
        let transformer = Transformer {
            query     : self.query.clone(),
            outvalue  : self.outvalue.clone(),
            transform : glue(self.transform, new_transform),
        };
        
        
        (transport, transformer)
    }
}


*/




