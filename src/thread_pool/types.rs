use std::hash::Hash;

pub trait RespTrait : Send + Sync + 'static {}
pub trait ParamTrait : Eq + Hash + RespTrait {}

pub type CounterType = u16;

pub type WorkerFunctionType<A: ParamTrait> = Box<Fn(A) + Send + Sync + 'static>;

pub type WorkerBuilderType<A: ParamTrait> = Box<Fn() -> WorkerFunctionType<A> + Send + Sync + 'static>;
