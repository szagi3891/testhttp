use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct SuperMutex<T> {
    mutex : Mutex<T>,
    cond  : Condvar,
}

impl<T> SuperMutex<T> {
    
    fn new(value: T) -> SuperMutex<T> {
        
        SuperMutex{
            mutex : Mutex::new(value),
            cond  : Condvar::new(),
        }
    }
}

fn main() {
    
    println!("test ...");
    
    //trzeba stworzyć super mutex ...
    
    let mu = Arc::new(SuperMutex::new(false));
    let mu2 = mu.clone();
    
    thread::spawn(move|| {
        let mut value = mu2.lock().unwrap();
        value.save(true);
    });
    
    
    let value = mu.lock().unwrap();
    while value.get() != true {
        value.wait().unwrap();
    }
    
    
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Inside of our lock, spawn a new thread, and then wait for it to start
    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    // wait for the thread to start up
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    
}