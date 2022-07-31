use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

mod lib;

use rust_web_fun::HTTPRes;

fn main() {

    let headers = vec![String::from("content-type: text/plain")];
    let body = String::from("hello you stupid ass dumb ass moron");
    let res = rust_web_fun::HTTPRes::make_response(1.1, 200, "OK".to_string(), headers, body);

    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &res);
    }
}

fn handle_connection(mut stream: TcpStream, res: &HTTPRes) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // stream.write(response.as_bytes()).unwrap();
    stream.write(res.format_response_string().as_bytes()).unwrap();

    stream.flush().unwrap();

}
