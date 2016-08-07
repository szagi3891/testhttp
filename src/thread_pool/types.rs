use std::hash::Hash;

pub trait RespTrait : Send + Sync + 'static {}
pub trait ParamTrait : Eq + Hash + RespTrait {}

pub type CounterType = u16;
