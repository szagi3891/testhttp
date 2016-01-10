use std::boxed::FnBox;
use std::thread::{self, JoinHandle};
use std::io::Result;

pub type Callback<T> = Box<FnBox(T) + Send + 'static + Sync>;

pub fn spawn<F, T>(name: String, block: F) -> Result<JoinHandle<T>>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
    
    thread::Builder::new().name(name).spawn(block)
}

/*
pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
    F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
    Builder::new().spawn(f)
}
*/
  
/*

let builder = createTaskBuilder(||{
    
    //ten clouser odpalany jest w momencie gdy zamknięte zostały wszystkie taski
});

builder ma jedną refernację do licznika tasków (który jest obiektem Arc)

gdy builder pójdzie w niepamięć, i gdy spadnie ilość tasków do zera, to wtedy odpalany jest clouser



mio server
    on może odpowiadać za licznik tasków

request.get_task()      -> referencja do taska


*/