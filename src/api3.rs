use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::mem::replace as mem_replace;

//use api1::get;

use api1;
use api2;

/*
 https://doc.rust-lang.org/std/cell/struct.RefCell.html
 https://doc.rust-lang.org/std/cell/

*/

pub fn get<F>(id: i32, job: F) where F: Fn(i32) + Send + 'static + Sync {

    //let job_box = Box::new(move |jobid: i32| job(jobid));
    let job_box = Box::new(job);

    struct ResultKK {
        job     : Option<Box<Fn(i32) + Send + 'static + Sync>>,
        result1 : Option<i32>,
        result2 : Option<i32>,
    }

    impl Drop for ResultKK {

        fn drop(&mut self) {
        
            let job = mem_replace(&mut self.job, None);

            job.unwrap()(100000 * self.result1.unwrap() + self.result2.unwrap());
            
        }
    }

    //let result = Arc::new(RwLock::new(Result::new()));
    let result = ResultKK{job: Some(job_box), result1: None, result2: None};
    let result = Arc::new(RwLock::new(result));

    let result_copy = result.clone();

    api1::get(50, move |res_data:i32| {

        println!("{} wykonuję callbacka 1", thread::current().name().unwrap());

        result_copy.write().unwrap().result1 = Some(res_data);
    });

    api2::get(1000, move |res_data: i32| {

        println!("{} wykonuję callbacka 2", thread::current().name().unwrap());

        result.write().unwrap().result2 = Some(res_data);
    });
}
