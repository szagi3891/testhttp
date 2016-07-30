use std::io;

pub type FilesData = Result<Vec<u8>, io::Error>;

pub struct Api_File {
    
}

impl Api_File {

    pub fn new() -> Api_File {
        Api_File{}
    }
}
