#[macro_use]
extern crate chan;
extern crate simple_signal;

use std::thread::{self,JoinHandle};
use std::io;
use simple_signal::{Signals, Signal};

fn main() {
    
    println!("Hello, world!");
    
    let (tx_request, rx_request) = chan::async();
    
    thread::spawn(move||{
        
        loop {
            
            println!("before send");
            tx_request.send("simple value1");
            tx_request.send("simple value2");
            tx_request.send("simple value3");
            println!("after send");
            
            thread::sleep_ms(50);
        }
    });
    
    
    
    
    let (tx_api_request , rx_api_request)  = chan::async();
    let (tx_api_response, rx_api_response) = chan::async();
    
    for i in 0..5 {
        
        let i_copy          = i.clone();
        let rx_api_request  = rx_api_request.clone();
        let tx_api_response = tx_api_response.clone();
        
        match spawn(format!("api #{}", &i_copy), move ||{
            
            loop {

                match rx_api_request.recv() {

                    Some(value) => {
                        
                        thread::sleep_ms(50);
                        tx_api_response.send(format!("response from api for request: {}", value));
                    }

                    None => {

                        println!("api none in channel");
                        return;
                    }
                }
            }
            
        }) {
            Ok(join_handle) => join_handle,
            Err(err) => panic!("Can't spawn api spawner: {}", err),
        };
    }
    
    
    
    for i in 0..30 {
        
        let i_copy          = i.clone();
        let rx_request      = rx_request.clone();
        let rx_api_response = rx_api_response.clone();
        let tx_api_request  = tx_api_request.clone();
        
        match spawn(format!("worker {}", &i_copy), move ||{
            
            loop {
                
                chan_select! {

                    rx_request.recv() -> val => {

                        println!("worker {}: get value rx_request {:?}", &i_copy, val);
                        tx_api_request.send(format!("request to api from worker {}", &i_copy));
                    },
                    
                    rx_api_response.recv() -> val => {
                        
                        println!("worker {}: get value rx_api_response {:?}", &i_copy, val);
                    }
                }
            }
        }) {
            Ok(join_handle) => join_handle,
            Err(err) => panic!("Can't spawn worker spawner: {}", err),
        };
    }
    
    
    let (ctrl_c_tx, ctrl_c_rx) = chan::sync(0);
    
    Signals::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
        
        println!("Termination signal catched.");
        ctrl_c_tx.send(());
    });
    
    
    let _ = ctrl_c_rx.recv();
    println!("Shutting down!");
}

fn spawn<F, T>(name: String, block: F) -> io::Result<JoinHandle<T>>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
    
    thread::Builder::new().name(name).spawn(block)
}
    
    