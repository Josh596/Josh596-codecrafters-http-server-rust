use regex::Regex;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use super::compression::CompressionType;

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
    pub is_complete: bool,
}

impl HTTPRequest {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, String> {
        let mut buf_reader = BufReader::new(stream);

        // Read headers
        let mut head_lines = String::new();
        let mut line = String::new();

        while let Ok(bytes_read) = buf_reader.read_line(&mut line) {
            if bytes_read == 0 || line.trim().is_empty() {
                break;
            }
            let l = line.len();
            println!("{} and length = {}", &line, l);
            head_lines.push_str(line.clone().as_str());
            line.clear();
        }
        let mut request = Self::from_head(&head_lines);

        if request.is_complete {
            return Ok(request);
        }

        if let Some(content_length) = request.headers.get("Content-Length") {
            let content_length: usize = content_length
                .parse()
                .map_err(|_| "Invalid Content-Length header")?;

            if content_length > 0 {
                let mut body = vec![0; content_length];
                buf_reader
                    .read_exact(&mut body)
                    .map_err(|e| format!("Failed to read body: {}", e))?;

                let compression_type = CompressionType::from_headers(&request.headers);
                let decoded_content = compression_type.decode(&body)?;
                request.body = String::from_utf8(decoded_content)
                    .map_err(|_| "Invalid UTF-8 in request body")?;
            }
        }
        request.is_complete = true;
        return Ok(request);
    }

    pub fn from_head(content: &str) -> Self {
        // let request_parts: Vec<&str> = content.split(" ").collect();
        // if request_parts.len()
        // Split using REGEX
        println!("{content}{}", content.len());
        let re =
            Regex::new(r"(?<method>\w+) (?<path>/?.+) (?<version>.+)\r\n(?<headers>(.+\r\n)*)")
                .unwrap();
        // let re = Regex::new(r"(?<method>\w+) (?<path>/?.+) (?<version>.+)").unwrap();
        let caps = re.captures(content).expect("Error Occurred");

        let method = &caps["method"];
        let path = &caps["path"];
        let version = &caps["version"];
        let headers_str = &caps["headers"];
        // let request_body = &caps["request_body"];
        // Parse headers here
        let mut headers = HashMap::new();
        let headers_re =
            Regex::new(r"(?:(?<header_name>.+):(?<header_value>[^(?:\r\n)]+))+").unwrap();

        for (_, [header_name, header_value]) in
            headers_re.captures_iter(headers_str).map(|c| c.extract())
        {
            headers.insert(
                header_name.trim().to_string(),
                header_value.trim().to_string(),
            );
        }

        let is_complete = match headers.get("Content-Length") {
            Some(content_length) if content_length.parse::<usize>().unwrap_or(0) > 0 => false,
            _ => true,
        };

        HTTPRequest {
            method: HTTPMethod::from_value(method).unwrap(),
            headers: headers,
            body: String::from(""),
            version: version.to_string(),
            path: path.to_string(),
            is_complete,
        }
    }
}
