use std::thread;
use std::time::Duration;

pub fn get<F>(id: i32, job: F) where F: FnOnce(i32) + Send + 'static {
    
    thread::spawn(move || {
        
        println!("get usypia");
        thread::sleep(Duration::new(2, 0));
        println!("get pobudka");
        
        job(id+ 777);
    });
}
