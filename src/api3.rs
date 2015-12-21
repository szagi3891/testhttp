use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

//use api1::get;

use api1;
use api2;

pub fn get<F>(id: i32, job: F) where F: FnOnce(i32) + Send + 'static + Sync {

    //let job_box = Box::new(move |jobid: i32| job(jobid));
    let job_box = Box::new(move |jobid: i32| job(jobid));

    struct Result {
        job     : Box<FnOnce(i32) + Send + 'static + Sync>,
        result1 : Option<i32>,
        result2 : Option<i32>,
    }

    impl Drop for Result {

        fn drop(&mut self) {

            //let job = self.job;
            self.job.call_once((100000 * self.result1.unwrap() + self.result2.unwrap(),));
            //println!("zbiorcze dane {:?} {:?}", self.result1, self.result2);
        }
    }

    //let result = Arc::new(RwLock::new(Result::new()));
    let result = Result{job: job_box, result1: None, result2: None};
    let result = Arc::new(RwLock::new(result));

    let result_copy = result.clone();

    api1::get(50, move |res_data:i32| {

        println!("wykonuję callbacka 1");

        result_copy.write().unwrap().result1 = Some(res_data);
    });

    api2::get(1000, move |res_data: i32| {

        println!("wykonuję callbacka 2");

        result.write().unwrap().result2 = Some(res_data);
    });
}
