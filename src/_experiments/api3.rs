use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::mem::replace as mem_replace;

//use api1::get;

use api1;
use api2;

/*

	do sprawdzenia pod kątem wyeliminowania mutex-a

 https://doc.rust-lang.org/std/cell/struct.RefCell.html
 https://doc.rust-lang.org/std/cell/

https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html


potem trzeba utworzyć z tego makro:
https://doc.rust-lang.org/book/macros.html

*/

pub fn get<F>(id: i32, job: F) where F: Fn(i32) + Send + 'static + Sync {

    let job_box = Box::new(job);

    struct ApiResult {
        job     : Option<Box<Fn(i32) + Send + 'static + Sync>>,
        result1 : Option<i32>,
        result2 : Option<i32>,
    }

    impl Drop for ApiResult {

        fn drop(&mut self) {
            let job = mem_replace(&mut self.job, None);
            job.unwrap()(100000 * self.result1.unwrap() + self.result2.unwrap());
        }

    }

    let result = ApiResult{job: Some(job_box), result1: None, result2: None};
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
