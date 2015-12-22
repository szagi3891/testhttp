use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use chan_signal::{Signal, notify};

struct watch {
    name : String,
    chan: Sender<String>,
}

impl watch {
    fn new(name: String, chan: Sender<String>) -> watch {    
        watch{name : name, chan: chan}
    }
}

impl Drop for watch {
    fn drop(&mut self) {
        println!("koniec wątka {}", &self.name);
        self.chan.send(self.name.clone());
    }
}


fn run_worker1(name: String, tx: Sender<String>) {
    
    thread::spawn(move || {
        
        let wat = watch::new(name, tx);
        
        //jakiś kod workera
        
        println!("1 usypia");
        thread::sleep(Duration::new(3, 0));
        println!("1 pobudka");
        
        panic!("panic1");
    });
}


fn run_worker2(name: String, tx: Sender<String>) {
    
    thread::spawn(move || {
        
        let wat = watch::new(name, tx);
        
        println!("2 usypia");
        thread::sleep(Duration::new(10, 0));
        println!("2 pobudka");
        
        panic!("panic2");
    });
}


pub fn test() {
    
    let (tx, rx) = channel();
    
    run_worker1("watek_pierwszy".to_string(), tx.clone());
    
    run_worker2("watek_drugi".to_string(), tx.clone());
    
    
    //od razu poeksperymentować z łapaniem sygnału ctrl+c - który posłuży do łagodnego wyjścia z programu
    
	//http://burntsushi.net/rustdoc/chan/macro.chan_select!.html
	
	let signal = notify(&[Signal::INT, Signal::TERM]);
	
	
    loop {
		
		chan_select! {
			
			sign = signal.recv() -> {
				
				println!("otrzymałem jakiś sygnał {:?}", sign);
			},
			
			message = rx.recv() -> {
				
				match message {
					
					Ok(thread_id) => {

						if thread_id == "watek_pierwszy".to_string() {

							run_worker1("watek_pierwszy".to_string(), tx.clone());

						} else if thread_id == "watek_drugi".to_string() {

							run_worker2("watek_drugi".to_string(), tx.clone());

						} else {

							panic!("nieprawidłowy identyfikator wątka");
						}

						//println!("otrzymałem informację że umarł wątek: {:?}", rx.recv());
					}

					Err(err) => {
						panic!("panik w przechwytywaniu z kanału {:?}");
					}
				}
			}
		}
		/*
		select! {
			
			sign = signal.recv() => {
				
				println!("otrzymałem jakiś sygnał {:?}", sign);
			},
			
			message = rx.recv() => {
				
				match message {
					
					Ok(thread_id) => {

						if thread_id == "watek_pierwszy".to_string() {

							run_worker1("watek_pierwszy".to_string(), tx.clone());

						} else if thread_id == "watek_drugi".to_string() {

							run_worker2("watek_drugi".to_string(), tx.clone());

						} else {

							panic!("nieprawidłowy identyfikator wątka");
						}

						//println!("otrzymałem informację że umarł wątek: {:?}", rx.recv());
					}

					Err(err) => {
						panic!("panik w przechwytywaniu z kanału {:?}");
					}
				}
			}
		}
		*/
    }
    
}