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
use std::collections::LinkedList;
use std::collections::vec_deque::VecDeque;


pub trait TransportIn<T> {
    fn send(self, T);       //TODO - tutaj będzie zwracana opcja na nowego sendera T2
}


pub struct Transport<T, R> {
    pub query  : Arc<Mutex<LinkedList<T>>>,
    pub query2 : Arc<Mutex<LinkedList<R>>>,
}

impl<T, R> TransportIn<T> for Transport<T, R> {
    
    fn send(self, value: T) {
        
        println!("wysyłam transportem wartość");
    }
}

fn chan<T: 'static>(val:T) {
    
    let mut list : VecDeque<Box<TransportIn<T>>> = VecDeque::new();
    
    let trans: Box<Transport<T,T>> = Box::new(Transport{
        query : Arc::new(Mutex::new(LinkedList::new())),
        query2 : Arc::new(Mutex::new(LinkedList::new())),
    });
    
    //list.push_back(Box::new(trans));
    trans.send(val);
    
    //list.push_back(1);
}

fn main() {
    
    //chan::<i32>(32);
    chan(32);
    
    println!("Hello, list world! 2");
}
