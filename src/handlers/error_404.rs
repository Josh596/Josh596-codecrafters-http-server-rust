use std::{collections::HashMap, net::TcpStream};

use crate::http::{request::HTTPRequest, response::HTTPResponse};
use regex::Regex;

use crate::context::HandlerContext;

pub fn error_404(
    ctx: &HandlerContext,
    stream: &mut TcpStream,
    request: &HTTPRequest,
) -> HTTPResponse {
    HTTPResponse {
        status_code: 404,
        status_text: String::from("Not Found"),
        headers: HashMap::new(),
        body: "".to_string(),
        version: String::from("HTTP/1.1"),
    }
}
