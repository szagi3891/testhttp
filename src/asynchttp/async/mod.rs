use std::boxed::FnBox;
use std::thread::{self, JoinHandle};
use std::io::Result;
use std::sync::{Arc, RwLock};
use std::mem::replace as mem_replace;
use std::collections::HashMap;

pub type Callback<T> = Box<FnBox(T) + Send + 'static + Sync>;


pub fn spawn<F, T>(name: String, block: F) -> Result<JoinHandle<T>>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
    
    thread::Builder::new().name(name).spawn(block)
}

            //TODO - do uruchomienia
/*
struct TaskJobEnd {
    job: Option<Box<Fn() + Send + 'static + Sync>>,
}

impl Drop for TaskJobEnd {

    fn drop(&mut self) {
        
        let job = mem_replace(&mut self.job, None);
        job.unwrap()();
    }
}

    
pub struct Task {
    job : Arc<RwLock<TaskJobEnd>>,
}

impl Task {
    
    pub fn new<F>(job: F) -> Task where F: Fn() + Send + 'static + Sync {
        
        let task = TaskJobEnd {
            job : Some(Box::new(job)),
        };
        
        Task {
            job : Arc::new(RwLock::new(task)),
        }
    }
    
    //pub fn copy() -> Task {}
}
*/

pub struct Manager {
    name   : String,
    len    : u32,
    count  : u32,
    map    : HashMap<i32, i32>,
    create : Box<Fn(String) + Send + 'static + Sync>,
}

impl Manager {
    
    pub fn new(name: String, len: u32, create: Box<Fn(String) + Send + 'static + Sync>) -> Manager {
        
        let mut instance = Manager {
            name   : name,
            len    : len,
            count  : 0,
            map    : HashMap::new(),
            create : create,
        };
        
        println!("!!!create");
        
        instance.refresh();
        
        instance
    }
    
    pub fn shoutdown(&mut self) {
        
    }
    
    fn refresh(&mut self) {
        
        loop {
            
            if self.len > self.count {
                
                let thread_name = self.name.clone();
                
                self.spawn(thread_name);      //TODO - trzeba będzie tworzyć nazwę nowego procesu
            
            } else if self.len < self.count {
                
                panic!("TODO - trzeba wyłączyć nadmiarową ilość wątków");
                
            } else {
                
                //a good amount
                return;
            }
        }
    }
    
    fn spawn(&mut self, name: String) {
        
        match &self.create {
            f => {
                f(name)
                /*
                match f(name) => {
                    Ok(join_handle) => {},
                    Err(err) => panic!("Can't spawn StaticHttp spawner: {}", err),
                };
                */
            }
        }
        
        self.count = self.count + 1;
    }
}



   
//id -> kanal
    
    
    //tutaj
    
    //kanał może przyjmować tylko "taski" typ
    //do stworzenia nowego tasku, potrzebny jest identyfikator tasku
    //request.task() - pobieramy nowy obiekt, zwiększa się licznik referencji z taskami
    //obiekt requestu do api
        //taskLicznik, enumRequestu
    
    /*
    
    api::Request(task_count.clone(), api::Request::GetFile(path_src.clone(), Callback(move|data: api::FilesData|{
    });
    */
    
    //tx_files_path.send((path_src.clone(), async::new(request.task(), |data: FilesData|{
    

/*
#![feature(unboxed_closures, box_syntax)]
#![allow(unstable)]

fn anyfunc<'a, Args, Ret, F>(closure: F) -> Box<Fn<Args, Ret> + 'a>
where F: Fn<Args, Ret> + 'a {
    box closure
}

fn main(){
    let celebrate = anyfunc(|&: num: i64| {
        println!("Feels great to implement C++ std::function in {} lines.", num)
    });
    let lucy_number = anyfunc(|&: num: i64| {
        println!("My lucky number is {}", num)
    });
    let func = if std::rand::random() { celebrate } else { lucy_number };
    func(25)
}
*/

/*
pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
    F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
    Builder::new().spawn(f)
}
*/
  
/*

let builder = createTaskBuilder(||{
    
    //ten clouser odpalany jest w momencie gdy zamknięte zostały wszystkie taski
});

builder ma jedną refernację do licznika tasków (który jest obiektem Arc)

gdy builder pójdzie w niepamięć, i gdy spadnie ilość tasków do zera, to wtedy odpalany jest clouser



mio server
    on może odpowiadać za licznik tasków

request.get_task()      -> referencja do taska


*/