#![feature(fnbox)]
#![feature(unboxed_closures)]

extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;
#[macro_use]
extern crate chan;

mod asynchttp;      //TODO - gdy się ustabilizuje, trzeba wynieść do zewnętrznego crate

mod app;            //przykładowa apka


fn main() {
    
    app::run_main();
}
