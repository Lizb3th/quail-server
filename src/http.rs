
#[cfg(test)]
mod http_tests;

mod response_handler;
mod request;
mod response;
mod request_handler;

pub type ResponseHandler = response_handler::ResponseHandler;
pub type RequestHandler = request_handler::RequestHandler;
type Request = request::Request;
pub type Response<F> = response::Response<F>;
pub type RequestGet<'a> = request::RequestGet<'a>;
pub type StatusLine = response::StatusLine;
pub type StatusCode = response::StatusCode;
pub type HeaderFields = response::HeaderFields;