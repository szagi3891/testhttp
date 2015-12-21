use api3;

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
