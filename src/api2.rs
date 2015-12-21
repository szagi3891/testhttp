use std::thread;
use std::time::Duration;

pub fn get<F>(id: i32, job: F) where F: FnOnce(i32) + Send + 'static {
    
    thread::spawn(move || {
        
        println!("get2 usypia");
        thread::sleep(Duration::new(3, 0));
        println!("get2 pobudka");
        
        job(id*4);
    });
}
