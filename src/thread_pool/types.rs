pub type CounterType = u16;

pub type WorkerFunctionType<A> = Box<Fn(A) + Send + Sync + 'static>;

pub type WorkerBuilderType<A> = Box<Fn() -> WorkerFunctionType<A> + Send + Sync + 'static>;
