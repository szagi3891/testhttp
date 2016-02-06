use std::sync::{Arc, Mutex, Condvar};
use std::collections::linked_list::LinkedList;

use transport::TransportOut;


pub enum GetResult<R> {
    List(LinkedList<Box<TransportOut<R> + Send>>),
    Value(R)
}



//TODO - te właściwości trzeba uprywatnić, dostęp do stanu ma się odbywać wyłącznie poprzez dedykowane metody

/*
    trzeba w pierwszym kroku zrobić żeby dostęp odbywał się tylko przez metody
    metoda, która będzie podmieniała zawartość outvalue
    
        docelowy efekt, wywołanie metody an outvalue powinno podmienić jej interpretację wewnętrzną
        kolejne puknięcie transportem z nową wartośćią powinno spowodować że transport zostanie transformowany w nowy transport
        wskazujący na nowy docelowy kanał
        
    outvalue . transform ( Fn(T) -> R)
        -> zwróci coś innego, ale mającego taki sam interfejs jak outvalue
        
        to nowe coś, będzie miało taką samą metodę jak originalne outvalue którą puka transport
            tylko że to coś będzie wykonywało transformację transportu
            
            
    !!!!!!!!!!!!!!!!!!!!!!! - prawdopodobny sposób na prawidłowego selecta
    
    pole outvalue wskazuje na transformera
        który nie ma typu - jest tylko interfejsem
    
    ten transformer tworzony jest na zewnątrz
    oraz wstrzykiwany w momencie gdy kanał jest transformowany
*/

pub struct Outvalue<R> {
    pub mutex : Mutex<OutvalueInner<R>>,
    pub cond  : Condvar,
}

impl<R> Outvalue<R> {
    
    pub fn new() -> Arc<Outvalue<R>> {
        
        Arc::new(Outvalue {
            mutex : OutvalueInner::new(),
            cond  : Condvar::new(),
        })
    }
    
    
    pub fn get(&self) -> GetResult<R> {

        let mut guard = self.mutex.lock().unwrap();

        let value = guard.take();

        match value {

            Some(value) => {
                return GetResult::Value(value);
            }
            None => {},
        }

        GetResult::List(guard.get_list_and_drain())
    }
    
    
    pub fn get_sync(&self) -> R {

        let mut guard = self.mutex.lock().unwrap();

        loop {
            
            let value = guard.take();

            match value {

                Some(value) => {
                    return value;
                }

                None => {

                    //println!("dalej pusta wartość w schowku, czekam dalej");
                }
            }

            guard = self.cond.wait(guard).unwrap();
        }
    }
}



//TODO zrobić te pola ukryte

pub struct OutvalueInner<R> {
    pub value : Option<R>,
    pub list  : LinkedList<Box<TransportOut<R> + Send>>,
}

impl<R> OutvalueInner<R> {
    
    fn new() -> Mutex<OutvalueInner<R>> {
        
        Mutex::new(OutvalueInner{
            value : None,
            list  : LinkedList::new(),
        })
    }
    
    fn take(&mut self) -> Option<R> {
        self.value.take()
    }
    
    
    fn get_list_and_drain(&mut self) -> LinkedList<Box<TransportOut<R> + Send>> {


        let mut out = LinkedList::new();

        loop {
            match self.list.pop_front() {
                Some(item) => out.push_back(item),
                None => return out
            }
        }
    }
}
