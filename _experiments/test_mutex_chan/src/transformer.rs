//use types::Param;
use std::sync::{Arc, Mutex};
use query::Query;
use outvalue::Outvalue;
use transport::{Transport, TransportOut};

/*
pub trait TransformerInterface<R> {
    fn transform<K>(self : Box<Self>, Box<Fn(R) -> K>) -> (TransportOut<K>, TransformerInterface<K>);
}
*/

/*
    trzeba wymyślić obiekt, który będzie zawierał clousera, lub referencję na kolejnego clousera
    (T) a (R)-> (R) b (K)
    
    na zewnątrz musi być to przykryte interfejsem parametryzującym po T i K
    czyli, efekt złączenia dwóch takich obiektów, musi finalnie na zewnątrz dawać nowy obiekt o zmienionym wyjściowym typie
*/


pub struct Transformer<T, R> {
    
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Outvalue<R>>,
    pub transform : Box<Fn(T) -> R + 'static + Send + Sync>,
}


impl<T, R> Transformer<T, R>
    where
        T : 'static + Send + Sync ,
        R : 'static + Send + Sync {
    
    fn transform<K>(self: Box<Self>, new_outvalue: &Arc<Outvalue<K>>, new_transform: &Fn() -> Box<Fn(R) -> K>) ->
        (Transport<T,K>, Transformer<T,K>)
        where K : 'static + Send + Sync {
        
        let transport = Transport {
            query    : self.query.clone(),
            outvalue : new_outvalue.clone(),
            transform : glue::<T,R,K>(self.transform, new_transform()),
        };
        
        let transformer = Transformer {
            query     : self.query.clone(),
            outvalue  : new_outvalue.clone(),
            transform : glue::<T,R,K>(self.transform, new_transform()),
        };
        
        
        (transport, transformer)
    }
}


fn glue<T,R,K>(fn1: Box<Fn(T) -> R + 'static + Send + Sync>, fn2: Box<Fn(R) -> K + 'static + Send + Sync>) -> Box<Fn(T) -> K + 'static + Send + Sync>
    where
        T : 'static + Send + Sync ,
        R : 'static + Send + Sync ,
        K : 'static + Send + Sync {
    
    Box::new(|arg: T| -> K {
        
        fn2(fn1(arg))
    })
}


