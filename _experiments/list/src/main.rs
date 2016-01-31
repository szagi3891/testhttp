/*

http://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait

trait Animal {
    fn make_sound(&self) -> String;
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "meow".to_string()
    }
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "woof".to_string()
    }
}

fn main () {
    let dog: Dog = Dog;
    let cat: Cat = Cat;
    let mut v: Vec<Box<Animal>> = Vec::new();
    v.push(Box::new(cat));
    v.push(Box::new(dog));
    for animal in v.iter() {
        println!("{}", animal.make_sound());
    }
}
*/



use std::sync::{Arc, Mutex};

pub trait TransportIn<T>  {
    fn send(self, T);
}

pub struct Transport<T, R> {
    pub query  : Arc<Mutex<Vec<T>>>,
    pub query2 : Arc<Mutex<Vec<R>>>,
}

impl<T, R> TransportIn<T> for Transport<T, R> {
    
    fn send(self, value: T) {
        
        println!("wysyłam transportem wartość");
    }
}

fn chan<T: 'static>(val:T) {
    
    let mut list : Vec<Box<TransportIn<T>>> = Vec::new();
    
    let trans: Box<Transport<T,T>> = Box::new(Transport{
        query : Arc::new(Mutex::new(Vec::new())),
        query2 : Arc::new(Mutex::new(Vec::new())),
    });
    
    list.push(trans);
    
    list.pop().unwrap().send(val);
}

fn main() {
    
    chan(32);
    
    println!("Hello, list world! 2");
}
