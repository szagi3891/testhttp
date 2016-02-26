/*
https://github.com/Detegr/rust-ctrlc
*/

use ctrlc::CtrlC;

pub fn signal_end(funk : Box<Fn() + Send + Sync + 'static>) {
    
    CtrlC::set_handler(move || {
        funk();
    });
}

/*
use simple_signal::{Signals, Signal};

fn signal_end(funk : Fn()) {
    
    Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
        
        funk();
    });
}
*/