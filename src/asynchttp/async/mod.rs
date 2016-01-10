use std::boxed::FnBox;
use std::thread::{self, JoinHandle};
use std::io::Result;
use std::sync::{Arc, RwLock};
use std::mem::replace as mem_replace;


pub type Callback<T> = Box<FnBox(T) + Send + 'static + Sync>;


pub fn spawn<F, T>(name: String, block: F) -> Result<JoinHandle<T>>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
    
    thread::Builder::new().name(name).spawn(block)
}


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
    
    /*
    pub fn copy() -> Task {
    }
    */
}


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