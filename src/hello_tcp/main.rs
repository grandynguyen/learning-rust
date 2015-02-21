#![feature(net)]
#![feature(io)]

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;


fn handle_client(mut stream: TcpStream) {
    let _ = stream.write(b"HTTP/1.0 200 OK\r\nContent-Type: text/html\r\n\r\n<html>
      <head><title>hello, world!</title></head><body>foo</body></html>");
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(_e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}
