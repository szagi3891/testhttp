#![feature(fnbox)]

extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;
#[macro_use]
extern crate chan;


// Module with macros defined should be stated first!
#[macro_use]
mod log;            //do zewnętrznego crates
mod async;          //do zewnętrznego crates
mod miohttp;        //do zewnętrznego crates

mod app;            //przykładowa apka


fn main() {
    
    app::run_main();
}
