use std::{
    io::{ prelude::*, BufReader },
    net::{ TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("connecting...");
        let stream = stream.unwrap();
        println!("connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
      .lines()
      .map(|result| result.unwrap())
      .take_while(|line| !line.is_empty())
      .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // http protocol requires sending data to be byte-format
    stream.write_all(response.as_bytes()).unwrap();
}