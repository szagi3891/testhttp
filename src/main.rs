#![feature(fnbox)]

//#![feature(plugin)]
//#![plugin(clippy)]

extern crate mio;
extern crate httparse;
extern crate time;
extern crate channels_async;
extern crate ctrlc;

mod signal_end;

mod asynchttp;      //TODO - gdy się ustabilizuje, trzeba wynieść do zewnętrznego crate

mod app;            //przykładowa apka


fn main() {
    
    app::run_main();
}


