struct Fnconvert<T,R> {
    
    funk : Box<Fn(T) -> R + 'static + Send + Sync>,
    builder : Fn() -> Box<Fn(T) -> R + Send + Sync + 'static>
}