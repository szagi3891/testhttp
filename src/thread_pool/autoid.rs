use thread_pool::types::{CounterType};

pub struct AutoId {
    id: CounterType
}

impl AutoId {
    pub fn new() -> AutoId {
        AutoId {
            id: 1
        }
    }
    
    pub fn get(&mut self) -> CounterType {

        let idClone = self.id.clone();
        self.id += 1;
        idClone
    }
}