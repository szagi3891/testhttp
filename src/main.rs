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


//(Request, &Token, mio::Sender<(mio::Token, response::Response)>)
    //zrobić z tego (Request, mioSender)
//mioSender.send(Response) - samozjadnie
//sprawdzać w dropie zawsze wysłano odpowiedź, jeśli nie wysłano to panic że jest błąd w logice kodu


fn main() {
    
    app::run_main();
}


