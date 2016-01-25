use std::sync::{Arc, Mutex, Condvar};

fn chan() {
}

struct In<T: Sized> {
    query : Arc<Mutex<StateQuery<T>>>,
}

struct StateQuery<T: Sized> {
    list : Vec<SenderIn<T>>,
}


struct Sender<T: Sized,R: Sized> {
    query : Arc<Mutex<StateQuery<T>>>,
    chan  : Arc<Mutex<StateChan<R>>>,
}

trait SenderIn<T: Sized> {
    fn send(self, T);       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}

/*
trait SenderOut {
}
*/

struct StateChan<R: Sized> {
    mutex : Mutex<Option<R>>,
    cond  : Condvar,
}

/*
impl<R> StateChan<R> {
    
    fn new() -> Arc<StateChan<R>> {
        Arc::new(StateChan{
            mutex : Mutex::new(),
            cond  : Condvar::new(),
        })
    }
    
    fn save() {
    }
    
    fn get() -> R {
        
    }
}
*/

//in
//stan
//sender
//stan kanału


fn main() {
    
    println!("test ... zx");
}