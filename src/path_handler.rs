use regex::Regex;
use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::{request::HTTPRequest, response::HTTPResponse};

pub fn handle_path(stream: &mut TcpStream, request: &HTTPRequest) {
    let path = &request.path;
    if path.starts_with("/echo") {
        let re = Regex::new(r"/(?<text>[^/]*)/?(?<message>.+)?").unwrap();
        let caps = re
            .captures(&path)
            .expect("Error Occurred while getting message");
        // dbg!(&caps["text"], &caps["message"], &path);
        let message = caps.name("message").map_or("", |m| m.as_str());
        let mut response = HTTPResponse {
            status_code: 200,
            status_text: String::from("OK"),
            headers: HashMap::new(),
            body: message.to_string(),
            version: String::from("HTTP/1.1"),
        };
        stream.write_all(response.construct().as_bytes());
    } else if path == "/user-agent" {
        let message = request.headers.get("User-Agent").unwrap();
        let mut response = HTTPResponse {
            status_code: 200,
            status_text: String::from("OK"),
            headers: HashMap::new(),
            body: message.to_string(),

            version: String::from("HTTP/1.1"),
        };
        stream.write_all(response.construct().as_bytes());
    } else if path == "/" {
        stream
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap();
    } else {
        stream
            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .unwrap();
    }
}
