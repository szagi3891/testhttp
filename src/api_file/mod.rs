use std::io::prelude::Read;
use std::fs::{self, File};
use std::path::Path;
use std::io;

use task_async::{self, Task, ThreadPool};

pub type FilesData = Result<Vec<u8>, io::Error>;

enum ParamFile {
    Get(String, Task<FilesData>),
}

pub struct ApiFile {
    thread_pool: ThreadPool<ParamFile>
}

impl ApiFile {

    pub fn new(workers_count: u16) -> ApiFile {
        
        let thread_pool = ThreadPool::new(workers_count, Box::new(move||{
            
            Box::new(move|param: ParamFile|{

                exec(param);
            })
        }));
    
        ApiFile{
            thread_pool: thread_pool
        }
    }
    
    pub fn get_file(&self, path_src: String, task: Task<FilesData>) {

        let param = ParamFile::Get(path_src, task);
        self.thread_pool.run(param);
    }
}

fn exec(param_file: ParamFile) {
    
    match param_file {
        ParamFile::Get(path, task) => {
            get_inner_file(path, task);
        }
    }
}

fn get_inner_file(path_src: String, task: Task<FilesData>) {

    let response = {

        let path = Path::new(&path_src);

        task_async::log_debug(format!("Loading file {:?}", path));

        match fs::metadata(path) {

            Ok(meta) => {

                // FIXME: Need to set a limit of max bytes read as na option maybe?
                if meta.len() > 1_000_000 {

                    task_async::log_error(format!("File {:?} is too big to serve", path));
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Static file too big"))

                } else {

                    match File::open(path) {

                        Ok(mut file) => {

                            let mut file_data: Vec<u8> = Vec::new();

                            match file.read_to_end(&mut file_data) {
                                Ok(_) => {
                                    task_async::log_debug(format!("Sending response ({} bytes).", file_data.len()));
                                    Ok(file_data)
                                }
                                Err(err) => Err(err),
                            }
                        },

                        Err(err) => Err(err),
                    }
                }
            }

            Err(err) => Err(err), 
        }
    };

    task.result(response);
}