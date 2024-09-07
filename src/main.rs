use regex::Regex;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};
enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HTTPMethod {
    fn from_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "get" => Ok(HTTPMethod::GET),
            "post" => Ok(HTTPMethod::POST),
            "put" => Ok(HTTPMethod::PUT),
            "delete" => Ok(HTTPMethod::DELETE),
            _ => Err(format!("Invalid Http Method -> {value}")),
        }
    }
}

struct HTTPRequest {
    method: HTTPMethod,
    headers: HashMap<String, String>, // Should be a HashMap
    body: String,
    path: String,
    version: String,
}

impl HTTPRequest {
    fn from_content(content: &str) -> Self {
        // let request_parts: Vec<&str> = content.split(" ").collect();
        // if request_parts.len()
        // Split using REGEX
        let re = Regex::new(
            r"(?<method>\w+) (?<path>/?.+) (?<version>.+)\r\n(?<headers>(?:.+\r\n)*)\r\n(?<request_body>.*)",
        )
        .unwrap();
        let caps = re.captures(content).expect("Error Occurred");

        let method = &caps["method"];
        let path = &caps["path"];
        let version = &caps["version"];
        let headers_str = &caps["headers"];
        let request_body = &caps["request_body"];

        // Parse headers here
        let mut headers = HashMap::new();
        let headers_re = Regex::new(r"(?:(?<header_name>.+):(?<header_value>[^\r\n]+))+").unwrap();

        for (_, [header_name, header_value]) in
            headers_re.captures_iter(headers_str).map(|c| c.extract())
        {
            println!("{}:{}", header_name, header_value);
            headers.insert(header_name.to_string(), header_value.to_string());
        }

        HTTPRequest {
            method: HTTPMethod::from_value(method).unwrap(),
            headers: headers,
            body: request_body.to_string(),
            version: version.to_string(),
            path: path.to_string(),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer).unwrap();
    let request = HTTPRequest::from_content(&buffer);

    let path = &request.path;
    // dbg!(path);
    match path.as_str() {
        "/" => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        }
        _ => {
            println!("Path not set");
            stream
                .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                .unwrap();
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(_stream);
                // _stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
