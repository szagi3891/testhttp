use std::boxed::FnBox;

pub type Callback<T> = Box<FnBox(T) + Send + 'static + Sync>;

