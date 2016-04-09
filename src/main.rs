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



// #[derive(Debug)]

//TODO - respchan       - trzeba zaimplementować dropa który będzie sprawdzał czy wysłana była odpowiedź, jeśli nie to ma rzucać panic



//TODO - request-a, można sklonować jeśli zajdzie potrzeba, ma być to niemutowalny parametr
        

//TODO - funkcję spawn, można by wsadzić do liba z taskami
    //funkcja spawn powinna współpracować z logowaniem
    //spawn powinno tworzyć ładne "drzewko"
    //natomiast logowanie powinno pozwalać na zgrupowanie logów względem poszczególnych wątków



fn main() {
    
    app::run_main();
}


