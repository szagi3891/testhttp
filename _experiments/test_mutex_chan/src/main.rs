use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct SuperMutex<T> {
    mutex : Mutex<Option<T>>,
    cond  : Condvar,
}

impl<T> SuperMutex<T> {
    
    fn new() -> SuperMutex<T> {
        
        SuperMutex{
            mutex : Mutex::new(None),
            cond  : Condvar::new(),
        }
    }
    
    fn save(&self, new_value: T) {
        
        let mut value = self.mutex.lock().unwrap();
        *value = Some(new_value);
        self.cond.notify_one();
        
        //notify_all?
    }
    
    fn get(&self) -> T {
        
        let mut value_opt = self.mutex.lock().unwrap();
        
        loop {
            
            let value = value_opt.take();
            
            match value {

                Some(value) => {
                    return value;
                }
                
                None => {
                    println!("dalej pusta wartość w schowku, czekam dalej");
                }
            }
            
            value_opt = self.cond.wait(value_opt).unwrap();
        }
    }
}

fn main() {
    
    println!("test ...");
    
    let mut mu1 = Arc::new(SuperMutex::new());
    let mut mu2 = mu1.clone();
    let mut mu3 = mu1.clone();
    
    thread::spawn(move|| {
        mu2.save("value thread1");
    });
    
    thread::spawn(move|| {
        mu3.save("value thread2");
    });
    
    let value_from_thread = mu1.get();
    println!("odebrana wartość z wątku {}", value_from_thread);
    
    let value_from_thread = mu1.get();
    println!("odebrana wartość z wątku {}", value_from_thread);
}


