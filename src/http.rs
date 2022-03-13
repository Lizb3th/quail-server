
#[cfg(test)]
mod http_tests;

mod response_handler;
mod request;

pub type ResponseHandler = response_handler::ResponseHandler;
type Request = request::Request;
pub type RequestGet<'a> = request::RequestGet<'a>;