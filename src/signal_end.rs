/*
https://github.com/Detegr/rust-ctrlc
*/

use ctrlc;
use task_async::callback0;
use channels_async::channel;


pub fn signal_end(func : callback0::CallbackBox) {
    
    let (send, recv) = channel();
    
    ctrlc::set_handler(move || {
        
        send.send(()).unwrap();
    });
    
    let _ = recv.get();
    
    func.exec();
}
