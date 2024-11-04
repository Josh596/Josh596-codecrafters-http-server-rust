use std::collections::HashMap;
use std::{io::Write, net::TcpStream};

pub struct HTTPResponse {
    pub status_code: u32,
    pub status_text: String,
    pub headers: HashMap<String, String>, // Should be a HashMap
    pub body: String,
    pub version: String,
}

impl HTTPResponse {
    // HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nabc
    pub fn construct(&mut self) -> String {
        // let version = self.version;

        let status_line = format!(
            "{version} {status_code} {status_text}",
            version = self.version,
            status_code = self.status_code,
            status_text = self.status_text
        );
        self.headers
            .entry(String::from("Content-Type"))
            .or_insert(String::from("text/plain"));

        self.headers
            .entry(String::from("Content-Length"))
            .or_insert(format!("{}", self.body.len()));

        let mut header_str = String::new();

        for key in self.headers.keys() {
            header_str.push_str(
                format!(
                    "{key}: {value}\r\n",
                    key = key,
                    value = self.headers.get(key).unwrap()
                )
                .as_str(),
            )
        }

        format!(
            "{status_line}\r\n{header}\r\n{body}",
            header = header_str,
            body = self.body
        )
        // let res = format!("{self.ver}");
    }

    pub fn send(&mut self, stream: &mut TcpStream) {
        stream.write_all(self.construct().as_bytes()).unwrap()
    }

    pub fn error_404() -> Self {
        HTTPResponse {
            status_code: 404,
            status_text: String::from("Not Found"),
            headers: HashMap::new(),
            body: "".to_string(),
            version: String::from("HTTP/1.1"),
        }
    }

    pub fn error_405() -> Self {
        HTTPResponse {
            status_code: 405,
            status_text: String::from("Method Not Allowed"),
            headers: HashMap::new(),
            body: "".to_string(),
            version: String::from("HTTP/1.1"),
        }
    }

    pub fn error_500() -> Self {
        HTTPResponse {
            status_code: 500,
            status_text: String::from("Internal Server Error"),
            headers: HashMap::new(),
            body: "".to_string(),
            version: String::from("HTTP/1.1"),
        }
    }
}
