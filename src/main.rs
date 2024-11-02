use codecrafters_http_server::start_server;
use std::env;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Get directory
    let args: Vec<String> = env::args().collect();

    let mut directory = String::from("");
    if args.len() > 2 {
        directory = args[2].clone();
    }

    start_server("127.0.0.1:4221", directory);
}
