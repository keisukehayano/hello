// main.rs

extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

// main関数
fn main() {
   // Listener IP Address, Port No.
   let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
   // threadPool 4個まで。。。
   let pool = ThreadPool::new(4);

   for stream in listener.incoming() {
       let stream = stream.unwrap();

       // 接続が確率しました。
       println!("Connection established!");

       pool.execute(|| {
        handle_connection(stream);
       });
   }

   println!("Shutting down.");
}

// handler connection
fn handle_connection(mut stream: TcpStream) {
    // 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    //　GET
    let get_index = b"GET / HTTP/1.1\r\n";
    let get_404_css = b"GET /404.css HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_index) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(get_404_css) {
        // 404.css
        ("HTTP/1.1 200 OK\r\n\r\n", "404.css")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = format!("{}{}",status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}