


mod api1 {
    
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
}

mod api2 {
    
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
}

mod api3 {
	
	use std::sync::{Arc, RwLock};
	use std::thread;
	use std::time::Duration;
	
	//use api1::get;
	
	pub fn get<F>(id: i32, job: F) where F: FnOnce(i32) + Send + 'static {
		
		struct Result {
			job     : FnOnce(i32) + Send + 'static,
			result1 : Option<i32>,
			result2 : Option<i32>,
		}
		
		impl Drop for Result {

			fn drop(&mut self) {
				
				self.job(100000 * self.result1 + self.result2);
				//println!("zbiorcze dane {:?} {:?}", self.result1, self.result2);
			}
		}

		//let result = Arc::new(RwLock::new(Result::new()));
		let result = Result{job: job, result1: None, result2: None};
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
	
}

pub fn test() {
	
	use std::thread;
	use std::time::Duration;
	
    println!("test z modułu async");
    
    api3::get(50, move |res:i32| {
		
		println!("zbiorcza odpowiedź {}", res);
	});
	
    println!("main, śpij");
    thread::sleep(Duration::new(5, 0));
    println!("main, pobudka");
}


	//let job_box = Box::new(move || job());
        

    /*
    pub fn execute<F>(&self, job: F)
        where F : FnOnce() + Send + 'static
    {
        self.jobs.send(Box::new(move || job())).unwrap();
    }
    
    let message = {
        // Only lock jobs for the time it takes
        // to get a job, not run it.
        let lock = jobs.lock().unwrap();
        lock.recv()
    };
    
    fn spawn_in_pool(jobs: Arc<Mutex<Receiver<Thunk<'static>>>>) {
        thread::spawn(move || {
            // Will spawn a new thread on panic unless it is cancelled.
            let sentinel = Sentinel::new(&jobs);

            loop {
                let message = {
                    // Only lock jobs for the time it takes
                    // to get a job, not run it.
                    let lock = jobs.lock().unwrap();
                    lock.recv()
                };

                match message {
                    Ok(job) => job.call_box(),

                    // The Threadpool was dropped.
                    Err(..) => break
                }
            }

            sentinel.cancel();
        });
    }
    
    pool.execute(move|| {
        tx.send(1).unwrap();
    });
    */
    
    
    /*
        pub fn defer<F>(&self, f: F) where F: FnOnce() + 'a {
        let mut dtors = self.dtors.borrow_mut();
        *dtors = Some(DtorChain {
            dtor: Box::new(f),
            next: dtors.take().map(Box::new)
        });
    }
    */




/*
//ob - me metodę ready

fn get_from_db(id, ob) {
    
}

//ob implementuje ready
fn get_right_column(ob) {
    
}


fn get_page(id, ob) {


    struct Result {
        result1
        result2
    }

    impl Result {
        
        fn ready1(data) {
            self.result1 = data
        }
        
        fn ready2(data) {
            self.result2 = data
        }
    }
    impl Drop for Result {
        fn drop(self) {
            //result1 i result2 są przetwarzane i generowana jest całościowa odpowiedź dla tej funkcji
            ob.ready(result);       //samo zjedzenie obiektu ob
        }
    }
    
	let result = Arc(result{});
	
	get_from_db(id, result.clone());
	        //ta funkcja powyższa powinna wywołać asynchronicznie ready1, funkcję
	
	get_right_column(result);
	        //ta funkcja poniższa powinna wywołać asynchronicznie funkcję ready2
}

fn main() {
    
    //odpalenie serwer-a www
    
    kanał // w ten kanał mamy wypchnąć asynchronicznie odpowiedź ze stroną gdy będzie wynik
    
    struct Str {
        kanal : kanał
    }
    
    impl Str {
        fn ready(self, wynik) {
            
            self.kanał <- wynik;
            
            //samozjedzenie self-a
            //powinno to dawać gwarancję że funkcja ready wykona się dokładnie raz
        }
    }
    
    get_page(32, Str{kanal : kanal});
}
*/