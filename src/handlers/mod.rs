use std::{collections::HashMap, fs, net::TcpStream};

use crate::http::{request::HTTPRequest, response::HTTPResponse};
use regex::Regex;

use crate::context::HandlerContext;
pub mod error_404;

pub fn echo(ctx: &HandlerContext, stream: &mut TcpStream, request: &HTTPRequest) -> HTTPResponse {
    let re = Regex::new(r"/(?<text>[^/]*)/?(?<message>.+)?").unwrap();
    let path = &request.path;

    let caps = re
        .captures(&path)
        .expect("Error Occurred while getting message");
    let message = caps.name("message").map_or("", |m| m.as_str());

    HTTPResponse {
        status_code: 200,
        status_text: String::from("OK"),
        headers: HashMap::new(),
        body: message.to_string(),
        version: String::from("HTTP/1.1"),
    }
}

pub fn user_agent(
    ctx: &HandlerContext,
    stream: &mut TcpStream,
    request: &HTTPRequest,
) -> HTTPResponse {
    let path = &request.path;
    let message = request.headers.get("User-Agent").unwrap();

    HTTPResponse {
        status_code: 200,
        status_text: String::from("OK"),
        headers: HashMap::new(),
        body: message.to_string(),

        version: String::from("HTTP/1.1"),
    }
}

pub fn index(ctx: &HandlerContext, stream: &mut TcpStream, request: &HTTPRequest) -> HTTPResponse {
    HTTPResponse {
        status_code: 200,
        status_text: String::from("OK"),
        headers: HashMap::new(),
        body: "".to_string(),
        version: String::from("HTTP/1.1"),
    }
}

pub fn files(ctx: &HandlerContext, stream: &mut TcpStream, request: &HTTPRequest) -> HTTPResponse {
    let path = &request.path;

    let file_path: Vec<&str> = path.split("/").collect();
    let filename = file_path.get(2).expect("Invalid Path");

    let full_path = ctx.base_dir().join(filename);
    // Read file content
    let file_content = fs::read_to_string(full_path);

    match file_content {
        Ok(content) => {
            let mut headers = HashMap::new();
            headers.insert(
                "Content-Type".to_string(),
                "application/octet-stream".to_string(),
            );
            return HTTPResponse {
                status_code: 200,
                status_text: String::from("OK"),
                headers: headers,
                body: content,
                version: String::from("HTTP/1.1"),
            };
        }
        Err(error) => return HTTPResponse::error_404(),
    }

    //
}
