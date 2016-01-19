use comm;

use asynchttp::miohttp::request::Request;

pub type RequestChannel<'a> = comm::mpmc::bounded::Channel<'a, Request>;

