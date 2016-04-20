/*
https://github.com/Detegr/rust-ctrlc
*/

use ctrlc::CtrlC;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use task_async::callback0;

pub fn signal_end(func : callback0::CallbackBox) {

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    CtrlC::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    });

    println!("Waiting for Ctrl-C... 0");
    while running.load(Ordering::SeqCst) {}

    println!("Got it! Exiting... 1");
    
    func.exec();
    
    println!("Got it! Exiting... 2");
}
