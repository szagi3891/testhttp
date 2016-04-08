//#![feature(plugin)]
//#![plugin(clippy)]

extern crate mio;
extern crate httparse;
extern crate time;
extern crate channels_async;
extern crate task_async;
extern crate ctrlc;

mod signal_end;

mod asynchttp;      //TODO - gdy się ustabilizuje, trzeba wynieść do zewnętrznego crate

mod app;            //przykładowa apka



//TODO - respchan       - trzeba zaimplementować dropa który będzie sprawdzał czy wysłana była odpowiedź, jeśli nie to ma rzucać panic


fn main() {
    
    app::run_main();
}


