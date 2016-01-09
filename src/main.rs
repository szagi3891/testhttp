#![feature(fnbox)]

extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;
#[macro_use]
extern crate chan;

mod async;
mod miohttp;
mod app;


fn main() {
    
    app::run_main();
}
