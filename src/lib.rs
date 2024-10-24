use path_handler::handle_path;
use pool::ThreadPool;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub mod request;
use crate::request::HTTPRequest;

pub mod response;

pub mod path_handler;
pub mod pool;

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_and_header: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Line count: {}", request_and_header.len());
    // Add CRLF, .lines() method removes the CRLF
    let request_and_header_lines = request_and_header.join("\r\n") + "\r\n\r\n";

    let request = HTTPRequest::from_content(&request_and_header_lines);

    handle_path(&mut stream, &request);
}

pub fn start_server(host: &str) {
    let listener = TcpListener::bind(host).unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
