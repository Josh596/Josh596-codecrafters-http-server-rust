use regex::Regex;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

pub mod request;
use crate::request::{HTTPMethod, HTTPRequest};

pub mod response;
use crate::response::HTTPResponse;
fn handle_connection(mut stream: TcpStream) {
    // let mut buffer = String::new();

    // let reader = BufReader::new(&stream);
    let mut buf: [u8; 1024] = [0; 1024];
    stream.read(&mut buf).unwrap();

    let s = String::from_utf8_lossy(&buf);
    // let content = reader.lines();
    // stream.read_to_string(&mut buffer).unwrap();

    // To parse http content
    // 1. Parse until /r/n/r/n
    let request = HTTPRequest::from_content(&s);

    let path = &request.path;
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

    // match message {
    //     "" => {
    //         stream
    //             .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
    //             .unwrap();
    //     }
    //     _ => {
    //         stream.write_all(response.construct().as_bytes());
    //     }
    // }

    if path.starts_with("/echo") {
        stream.write_all(response.construct().as_bytes());
    }

    match path.as_str() {
        "/" => {
            stream
                .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                .unwrap();
        }
        _ => {
            stream
                .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                .unwrap();
        }
    };
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
                // _stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                println!("handled new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
