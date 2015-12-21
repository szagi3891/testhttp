use api3;

pub fn test() {
	
	use std::thread;
	use std::time::Duration;
	
    println!("test z modułu async");
    
    api3::get(50, move |res:i32| {
		
		println!("{} zbiorcza odpowiedź {}", thread::current().name().unwrap(), res);
	});
	
    println!("main, śpij");
    thread::sleep(Duration::new(5, 0));
    println!("main, pobudka");
}


/*
3:19:59 < szaman> http://is.gd/fEjcyO - how to invoke 'job'  in the drop implementation ? is that possible at all?
13:20:49 ! edoput [edoput@moz-8js.ari.40.151.IP] has joined #rust-beginners
13:24:04 ! Eber [Eber@moz-nal.l32.80.177.IP] has left #rust-beginners []
13:24:06 ! huguex [huguex@moz-njg.qpn.252.103.IP] has joined #rust-beginners
13:24:11 < bur_sangjun> szaman: I'm not fully understanding what you're trying to achieve
13:26:04 < szaman> bur_sangjun: while dropping JobResult, I want to run 'job' closure
13:30:01 ! DroidLogician [Austin@moz-9lhp5e.ca.charter.com] has quit [Quit: Leaving]
13:30:46 ! Andris_zbx [andris@moz-9cq.e6i.110.87.IP] has joined #rust-beginners
13:33:10 < bur_sangjun> szaman: Ah, your problem is then that FnOnce requires ownership, 
13:33:19 < bur_sangjun> szaman: Swithc it out for FnMut which doesn't, and it will work
13:33:48 < bur_sangjun> http://is.gd/S8EdId
13:35:09 < bur_sangjun> szaman: Note: Fn will also work here, as you don't actually need any mutation that would be provided by FnMut
13:35:16 < bur_sangjun> szaman: So your problem is you are marking the closure as FnOnce
13:36:13 ! mcint [mcint@moz-6hr3c9.swbr.surewest.net] has quit [Quit: hibernating...]
13:36:28 < bur_sangjun> basically, in order to call an FnOnce closure, you need to own it, in order to call an Fn closure, you need a reference to it, and in order to call an FnMut closure, you need a borrow of it
13:37:34 < szaman> thanks! i'll check it
13:38:59 < bur_sangjun> szaman: The rough type signatures are `FnOnce::call_once(self, args: Args) -> Output`, `Fn::call(&self, args: Args) -> Output` and `FnMut::call_mut(&mut self, args: Args) -> Output`

*/


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
