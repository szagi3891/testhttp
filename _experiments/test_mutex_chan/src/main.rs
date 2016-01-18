use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct Channel<T> {
    buffor : Vec
    //tablica z recivierami
}

struct Sender {
    chan : Arc<Mutex<Channel>>
}
/*
    sender wysyłając, blokuje cały Channel
    następnie dokłada do bufora,
    potem sprawdza, czy są jakieś oczekujące na dane kanały, jeśli są, to próbuje je wszystkie zasilić danymi
*/

struct Recivier<T> {
    chan : Arc<Mutex<Channel<T>>>
    data : Arc<(Mutex(T), Condvar())>       //do synchronizacji wątków, czyli wskrzeszenie czekającego zaraz po pojawieniu się danej
}

impl Recivier {
    reciv() -> T {
    }
}

fn make_chan() -> (Sender, Recivier) {
    
    let chan = Channel{
    };
    
    //dwie kopie tego samego recivier-a się tworzy
    //pierwsza kopia trafia do tablicy z recivierami tego kanału
    //druga kopia idzie do uzytkownika końcowego
    
    //dobrze by było, jeśli druga kopia przy niszczeniu, powodowałaby usuwanie pierwszej kopi z kanału
    
    (chan.sender(), chan.recivier())
}



fn main() {
    
    /*
    let (tx, rx) = chan::new();
    
    
    struktura
        save (T) - zapisanie w schowek kanałowy
    
    
    struct Channel {
        bufor   []
    }

    struct Recivier {
        data : Arc(Mutex(data)) <T>         -> data.save(T) -> Option<T>, zapis danej do kanału
    }
    
    struct Sender {
        ref : klon na schowek
    }
    
    wysyłając dane, sender robi locka, a następnie próbuje włożyć dane do schowka
    */
    
    
    println!("Hello, world!");
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Inside of our lock, spawn a new thread, and then wait for it to start
    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        println!("wysyłam notify");
        cvar.notify_one();
    });
    
    
    // wait for the thread to start up
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("dalej");
}
