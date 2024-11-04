use pool::ThreadPool;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{net::TcpStream, path::PathBuf};

pub mod http;
use crate::http::request::HTTPRequest;
pub mod context;
pub mod handlers;
pub mod path_handler;
pub mod pool;
pub fn handle_connection(mut stream: TcpStream, base_dir: String) {
    // let request_and_header: Vec<String> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Line count: {}", request_and_header.len());
    // Add CRLF, .lines() method removes the CRLF

    // let request = HTTPRequest::from_head(&request_and_header_lines);
    let request = HTTPRequest::from_stream(&mut stream).expect("Unable to parse HTTp Request");
    let handler = path_handler::setup_path_handler(PathBuf::from(base_dir));

    handler
        .handle_request(&request, &mut stream)
        .expect("Error occured while handling request");
}

pub fn start_server(host: &str, base_dir: String) {
    let listener = TcpListener::bind(host).unwrap();

    // let base_dir = Arc::new(Mutex::new(base_dir));
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let cloned_base_dir = base_dir.clone();
        pool.execute(|| {
            handle_connection(stream, cloned_base_dir);
        });
    }
}
