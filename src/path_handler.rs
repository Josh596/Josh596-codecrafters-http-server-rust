use regex::Regex;
use std::{collections::HashMap, io::Write, net::TcpStream, path::PathBuf};

use crate::{
    context::HandlerContext,
    http::{request::HTTPRequest, response::HTTPResponse},
};

type CallbackHandler = Box<dyn Fn(&HandlerContext, &mut TcpStream, &HTTPRequest) -> HTTPResponse>;

struct Path {
    pattern: Regex,
    callback: CallbackHandler,
}
pub struct PathHandler {
    paths: Vec<Path>,
    context: HandlerContext,
}

impl PathHandler {
    fn new(context: HandlerContext) -> Self {
        PathHandler {
            paths: Vec::new(),
            context,
        }
    }
    pub fn register_path<F>(&mut self, path_regex: &str, callback: F) -> Result<(), regex::Error>
    where
        F: Fn(&HandlerContext, &mut TcpStream, &HTTPRequest) -> HTTPResponse + 'static,
    {
        let path = Regex::new(&path_regex).unwrap();
        self.paths.push(Path {
            pattern: path,
            callback: Box::new(callback),
        });
        Ok(())
    }

    pub fn handle_request(
        &self,
        request: &HTTPRequest,
        stream: &mut TcpStream,
    ) -> Result<(), String> {
        // Searches through path till it finds a match.
        let path = &request.path;
        for registered_path in &self.paths {
            if registered_path.pattern.is_match(path) {
                println!(
                    "A match has been found; handler => {}, path => {path}",
                    registered_path.pattern
                );
                let mut response = (registered_path.callback)(&self.context, stream, request);
                stream.write_all(response.construct().as_bytes()).unwrap();

                return Ok(());
            }
        }

        let mut response = crate::handlers::error_404::error_404(&self.context, stream, request);
        stream.write_all(response.construct().as_bytes()).unwrap();

        Err(format!("404: Path {} is not registered", &request.path))
    }
}

#[allow(unused)]
pub fn setup_path_handler(base_dir: PathBuf) -> PathHandler {
    let context = HandlerContext::new(base_dir);
    let mut handler = PathHandler::new(context);

    handler.register_path("/echo", crate::handlers::echo);
    handler.register_path("/user-agent", crate::handlers::user_agent);
    handler.register_path(r"/files/.*", crate::handlers::files);
    handler.register_path("^/$", crate::handlers::index);

    handler
}
