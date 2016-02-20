use std::sync::Arc;
use transformer::Transformer;
use outvalue::Outvalue;
use transport::Transport;


pub struct Receiver<T, R> {
    pub outvalue : Arc<Outvalue<R>>,
    transformer  : Transformer<T, R>,
}


impl<T,R> Receiver<T,R>
    where
        T : Send + Sync + 'static ,
        R : Send + Sync + 'static {
    
    pub fn new(outvalue: Arc<Outvalue<R>>, transformer: Transformer<T, R>) -> Receiver<T, R> {
        Receiver{
            outvalue    : outvalue,
            transformer : transformer
        }
    }
    
    pub fn transform<K>(self, outvalue: Arc<Outvalue<K>>, trans_fn: Box<Fn(R) -> K + Send + Sync + 'static>) -> Transport<T,K>
        where K : Send + Sync + 'static {
        
        self.transformer.transform(outvalue, trans_fn)
    }
    
    pub fn get(&self) -> R {
        
        self.outvalue.get()
    }
}
