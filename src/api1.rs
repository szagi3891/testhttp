use std::thread;
use std::time::Duration;

pub fn get<F>(id: i32, job: F) where F: FnOnce(i32) + Send + 'static {
    
    thread::Builder::new().name("api1".to_string()).spawn(move || {
        println!("{} usypia", thread::current().name().unwrap());
        thread::sleep(Duration::new(2, 0));
        println!("{} pobudka", thread::current().name().unwrap());
        
        job(id+ 777);
    });

}
