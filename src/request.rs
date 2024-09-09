use regex::Regex;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HTTPMethod {
    pub fn from_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "get" => Ok(HTTPMethod::GET),
            "post" => Ok(HTTPMethod::POST),
            "put" => Ok(HTTPMethod::PUT),
            "delete" => Ok(HTTPMethod::DELETE),
            _ => Err(format!("Invalid Http Method -> {value}")),
        }
    }
}

pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub headers: HashMap<String, String>, // Should be a HashMap
    pub body: String,
    pub path: String,
    pub version: String,
}

impl HTTPRequest {
    pub fn from_content(content: &str) -> Self {
        // let request_parts: Vec<&str> = content.split(" ").collect();
        // if request_parts.len()
        // Split using REGEX
        println!("{content} {}", content.len());
        let re = Regex::new(
            r"(?<method>\w+) (?<path>/?.+) (?<version>.+)\r\n(?<headers>(?:.+\r\n)*)\r\n(?<request_body>.*)",
        )
        .unwrap();
        // let re = Regex::new(r"(?<method>\w+) (?<path>/?.+) (?<version>.+)").unwrap();
        let caps = re.captures(content).expect("Error Occurred");

        let method = &caps["method"];
        let path = &caps["path"];
        let version = &caps["version"];
        let headers_str = &caps["headers"];
        let request_body = &caps["request_body"];

        // Parse headers here
        let mut headers = HashMap::new();
        let headers_re =
            Regex::new(r"(?:(?<header_name>.+):(?<header_value>[^(?:\r\n)]+))+").unwrap();

        for (_, [header_name, header_value]) in
            headers_re.captures_iter(headers_str).map(|c| c.extract())
        {
            println!("{}:{}", header_name, header_value);
            headers.insert(
                header_name.trim().to_string(),
                header_value.trim().to_string(),
            );
        }

        // let request_body = "";

        HTTPRequest {
            method: HTTPMethod::from_value(method).unwrap(),
            headers: headers,
            body: request_body.to_string(),
            version: version.to_string(),
            path: path.to_string(),
        }
    }
}
