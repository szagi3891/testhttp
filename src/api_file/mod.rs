use std::io::prelude::Read;
use std::fs::{self, File};
use std::path::Path;
use std::io;

use task_async::{self, Task};


pub type FilesData = Result<Vec<u8>, io::Error>;

pub struct ApiFile {
}

impl ApiFile {

    pub fn new() -> ApiFile {
        
        //trzeba stworzyć workery obsługujące czytanie plików
        
        ApiFile{}
    }
    
    pub fn get_file(&self, path_src: String, task: Task<FilesData>) {

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
}
