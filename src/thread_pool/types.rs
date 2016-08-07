use std::hash::Hash;

//trait RespTrait : Send + Sync + 'static {}
//pub trait ParamTrait : Eq + Hash + RespTrait {}

pub trait ParamTrait : Eq + Hash + Send + Sync + 'static {}

pub type CounterType = u16;

pub type WorkerFunctionType<A: ParamTrait> = Box<Fn(A) + Send + Sync + 'static>;

pub type WorkerBuilderType<A: ParamTrait> = Box<Fn() -> WorkerFunctionType<A> + Send + Sync + 'static>;
