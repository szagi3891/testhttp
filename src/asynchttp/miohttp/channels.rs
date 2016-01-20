use comm;

use asynchttp::miohttp::request::Request;

//pub type RequestChannel<'a> = comm::mpmc::bounded::Channel<'a, Request>;  // performance problems with nthreads > cores
//pub type RequestChannel<'a> = comm::spmc::bounded_fast; //::new(100);  // unsafe

pub type RequestProducer<'a> = comm::spmc::unbounded::Producer<'a, Request>;
pub type RequestConsumer<'a> = comm::spmc::unbounded::Consumer<'a, Request>;

pub fn new_request_channel<'a>() -> (RequestProducer<'a>, RequestConsumer<'a>) {
    return comm::spmc::unbounded::new();          // Overflow possibility on unbounded channel
}
