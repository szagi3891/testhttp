fn chan() {
}

struct In {
    query : Arc<Mutex<StateQuery>>,
}

struct StateQuery {
    list : Vec<Sender>,
}

struct Sender {
    query : Arc<Mutex<In>>,
    chan  : Arc<Mutex<StateChan>>,
}

struct StateChan {
    name : String
}

//in
//stan
//sender
//stan kanału


fn main() {
    
    println!("test ... zx");
}