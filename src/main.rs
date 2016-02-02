#![feature(fnbox)]
#![feature(unboxed_closures)]
#![feature(clone_from_slice)]

extern crate mio;
extern crate simple_signal;
extern crate httparse;
extern crate time;
extern crate comm;
extern crate inlinable_string;

mod asynchttp;      //TODO - gdy się ustabilizuje, trzeba wynieść do zewnętrznego crate

mod app;            //przykładowa apka


fn main() {
    
    app::run_main();
}
